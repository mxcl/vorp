use std::{
    collections::{HashMap, HashSet},
    fmt,
    ops::Range,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
    time::Duration,
};

use string_offset::ByteOffset;
use thiserror::Error;
use warpui::{AppContext, Entity, ModelContext, SingletonEntity};

use crate::{
    index::locations::CodeContextLocation,
    warp_graphql,
    warp_graphql::queries::rerank_fragments::FragmentLocationInput,
    workspace::{WorkspaceMetadata, WorkspaceMetadataEvent},
};

pub mod manager {
    pub use super::{
        BuildSource, CodebaseIndexFinishedStatus, CodebaseIndexManager, CodebaseIndexManagerEvent,
        CodebaseIndexStatus, CodebaseIndexingError, RetrieveFileError,
    };
}

pub mod store_client {
    use super::*;
    use async_trait::async_trait;

    #[cfg_attr(not(target_family = "wasm"), async_trait)]
    #[cfg_attr(target_family = "wasm", async_trait(?Send))]
    pub trait StoreClient: 'static + Send + Sync {
        async fn update_intermediate_nodes(
            &self,
            embedding_config: EmbeddingConfig,
            nodes: Vec<IntermediateNode>,
        ) -> Result<HashMap<NodeHash, bool>, Error>;

        async fn generate_embeddings(
            &self,
            embedding_config: EmbeddingConfig,
            fragments: Vec<Fragment>,
            root_hash: NodeHash,
            repo_metadata: RepoMetadata,
        ) -> Result<HashMap<ContentHash, bool>, Error>;

        async fn populate_merkle_tree_cache(
            &self,
            embedding_config: EmbeddingConfig,
            root_hash: NodeHash,
            repo_metadata: RepoMetadata,
        ) -> Result<bool, Error>;

        async fn sync_merkle_tree(
            &self,
            nodes: Vec<NodeHash>,
            embedding_config: EmbeddingConfig,
        ) -> Result<HashSet<NodeHash>, Error>;

        async fn rerank_fragments(
            &self,
            query: String,
            fragment: Vec<Fragment>,
        ) -> Result<Vec<Fragment>, Error>;

        async fn get_relevant_fragments(
            &self,
            embedding_config: EmbeddingConfig,
            query: String,
            root_hash: NodeHash,
            repo_metadata: RepoMetadata,
        ) -> Result<Vec<ContentHash>, Error>;

        async fn codebase_context_config(&self) -> Result<CodebaseContextConfig, Error>;
    }

    #[derive(Debug, Default, Clone)]
    pub struct MockStoreClient;

    #[cfg_attr(not(target_family = "wasm"), async_trait)]
    #[cfg_attr(target_family = "wasm", async_trait(?Send))]
    impl StoreClient for MockStoreClient {
        async fn update_intermediate_nodes(
            &self,
            _embedding_config: EmbeddingConfig,
            _nodes: Vec<IntermediateNode>,
        ) -> Result<HashMap<NodeHash, bool>, Error> {
            Ok(HashMap::new())
        }

        async fn generate_embeddings(
            &self,
            _embedding_config: EmbeddingConfig,
            _fragments: Vec<Fragment>,
            _root_hash: NodeHash,
            _repo_metadata: RepoMetadata,
        ) -> Result<HashMap<ContentHash, bool>, Error> {
            Ok(HashMap::new())
        }

        async fn populate_merkle_tree_cache(
            &self,
            _embedding_config: EmbeddingConfig,
            _root_hash: NodeHash,
            _repo_metadata: RepoMetadata,
        ) -> Result<bool, Error> {
            Ok(true)
        }

        async fn sync_merkle_tree(
            &self,
            _nodes: Vec<NodeHash>,
            _embedding_config: EmbeddingConfig,
        ) -> Result<HashSet<NodeHash>, Error> {
            Ok(HashSet::new())
        }

        async fn rerank_fragments(
            &self,
            _query: String,
            fragments: Vec<Fragment>,
        ) -> Result<Vec<Fragment>, Error> {
            Ok(fragments)
        }

        async fn get_relevant_fragments(
            &self,
            _embedding_config: EmbeddingConfig,
            _query: String,
            _root_hash: NodeHash,
            _repo_metadata: RepoMetadata,
        ) -> Result<Vec<ContentHash>, Error> {
            Ok(Vec::new())
        }

        async fn codebase_context_config(&self) -> Result<CodebaseContextConfig, Error> {
            Ok(CodebaseContextConfig {
                embedding_config: EmbeddingConfig::default(),
                embedding_cadence: Duration::from_secs(300),
            })
        }
    }

    #[derive(Debug, Clone)]
    pub struct IntermediateNode {
        pub hash: NodeHash,
        pub children: Vec<NodeHash>,
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid hash: {0:#}")]
    InvalidHash(base16ct::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EmbeddingConfig {
    OpenAiTextSmall3_256,
    VoyageCode3_512,
    Voyage3_5_Lite_512,
    #[default]
    Voyage3_5_512,
}

impl From<EmbeddingConfig> for warp_graphql::full_source_code_embedding::EmbeddingConfig {
    fn from(val: EmbeddingConfig) -> Self {
        match val {
            EmbeddingConfig::OpenAiTextSmall3_256 => {
                warp_graphql::full_source_code_embedding::EmbeddingConfig::OpenaiTextSmall3256
            }
            EmbeddingConfig::VoyageCode3_512 => {
                warp_graphql::full_source_code_embedding::EmbeddingConfig::VoyageCode3512
            }
            EmbeddingConfig::Voyage3_5_512 => {
                warp_graphql::full_source_code_embedding::EmbeddingConfig::Voyage35512
            }
            EmbeddingConfig::Voyage3_5_Lite_512 => {
                warp_graphql::full_source_code_embedding::EmbeddingConfig::Voyage35Lite512
            }
        }
    }
}

impl TryFrom<warp_graphql::full_source_code_embedding::EmbeddingConfig> for EmbeddingConfig {
    type Error = Error;

    fn try_from(
        value: warp_graphql::full_source_code_embedding::EmbeddingConfig,
    ) -> Result<Self, Self::Error> {
        match value {
            warp_graphql::full_source_code_embedding::EmbeddingConfig::OpenaiTextSmall3256 => {
                Ok(Self::OpenAiTextSmall3_256)
            }
            warp_graphql::full_source_code_embedding::EmbeddingConfig::Voyage35Lite512 => {
                Ok(Self::Voyage3_5_Lite_512)
            }
            warp_graphql::full_source_code_embedding::EmbeddingConfig::VoyageCode3512 => {
                Ok(Self::VoyageCode3_512)
            }
            warp_graphql::full_source_code_embedding::EmbeddingConfig::Voyage35512 => {
                Ok(Self::Voyage3_5_512)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeHash(String);

impl fmt::Display for NodeHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for NodeHash {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        validate_hash(s)?;
        Ok(Self(s.to_owned()))
    }
}

impl From<NodeHash> for warp_graphql::full_source_code_embedding::NodeHash {
    fn from(value: NodeHash) -> Self {
        warp_graphql::full_source_code_embedding::NodeHash(value.0)
    }
}

impl TryFrom<warp_graphql::full_source_code_embedding::NodeHash> for NodeHash {
    type Error = Error;

    fn try_from(
        value: warp_graphql::full_source_code_embedding::NodeHash,
    ) -> Result<Self, Self::Error> {
        value.0.parse()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContentHash(String);

impl fmt::Display for ContentHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for ContentHash {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        validate_hash(s)?;
        Ok(Self(s.to_owned()))
    }
}

impl From<ContentHash> for warp_graphql::full_source_code_embedding::ContentHash {
    fn from(value: ContentHash) -> Self {
        warp_graphql::full_source_code_embedding::ContentHash(value.0)
    }
}

impl TryFrom<warp_graphql::full_source_code_embedding::ContentHash> for ContentHash {
    type Error = Error;

    fn try_from(
        value: warp_graphql::full_source_code_embedding::ContentHash,
    ) -> Result<Self, Self::Error> {
        value.0.parse()
    }
}

fn validate_hash(value: &str) -> Result<(), Error> {
    let mut buf = [0u8; 32];
    let decoded =
        base16ct::lower::decode(value.as_bytes(), &mut buf).map_err(Error::InvalidHash)?;
    if decoded.len() != 32 {
        return Err(Error::InvalidHash(base16ct::Error::InvalidLength));
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct RepoMetadata {
    pub path: Option<String>,
}

impl From<RepoMetadata> for warp_graphql::full_source_code_embedding::RepoMetadata {
    fn from(val: RepoMetadata) -> Self {
        Self { path: val.path }
    }
}

#[derive(Clone, Copy)]
pub struct CodebaseContextConfig {
    pub embedding_config: EmbeddingConfig,
    pub embedding_cadence: Duration,
}

#[derive(Clone)]
pub struct FragmentLocation {
    absolute_path: PathBuf,
    byte_range: Range<ByteOffset>,
}

#[derive(Clone)]
pub struct Fragment {
    content: String,
    content_hash: ContentHash,
    location: FragmentLocation,
}

impl From<Fragment> for warp_graphql::full_source_code_embedding::Fragment {
    fn from(val: Fragment) -> Self {
        Self {
            content: val.content,
            content_hash: val.content_hash.into(),
        }
    }
}

impl From<Fragment> for warp_graphql::queries::rerank_fragments::RerankFragmentInput {
    fn from(val: Fragment) -> Self {
        Self {
            content: val.content,
            content_hash: val.content_hash.into(),
            location: FragmentLocationInput {
                byte_start: val.location.byte_range.start.as_usize() as i32,
                byte_end: val.location.byte_range.end.as_usize() as i32,
                file_path: val.location.absolute_path.to_string_lossy().to_string(),
            },
        }
    }
}

impl TryFrom<warp_graphql::queries::rerank_fragments::RerankFragment> for Fragment {
    type Error = Error;

    fn try_from(
        val: warp_graphql::queries::rerank_fragments::RerankFragment,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            content: val.content,
            content_hash: val.content_hash.try_into()?,
            location: FragmentLocation {
                absolute_path: PathBuf::from(val.location.file_path),
                byte_range: ByteOffset::from(val.location.byte_start as usize)
                    ..ByteOffset::from(val.location.byte_end as usize),
            },
        })
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RetrievalID(usize);

#[derive(Debug, Copy, Clone)]
pub enum SyncProgress {
    Discovering {
        total_nodes: usize,
    },
    Syncing {
        completed_nodes: usize,
        total_nodes: usize,
    },
}

pub enum SyncTask {}

pub enum CodebaseIndexFinishedStatus {
    Completed,
    Failed(CodebaseIndexingError),
}

#[derive(Error, Debug)]
pub enum RetrieveFileError {
    #[error("Codebase index still indexing")]
    IndexSyncing,
    #[error("Codebase index failed: {0:#}")]
    IndexFailed(CodebaseIndexingError),
    #[error("Codebase index not found")]
    IndexNotFound,
}

pub enum CodebaseIndexManagerEvent {
    RetrievalRequestCompleted {
        retrieval_id: RetrievalID,
        fragments: Arc<HashSet<CodeContextLocation>>,
        out_of_sync_delay: Option<Duration>,
    },
    RetrievalRequestFailed {
        retrieval_id: RetrievalID,
        error_message: String,
    },
    SyncStateUpdated,
    IndexMetadataUpdated {
        root_path: PathBuf,
        event: WorkspaceMetadataEvent,
    },
    RemoveExpiredIndexMetadata {
        expired_metadata: Arc<Vec<PathBuf>>,
    },
    NewIndexCreated,
}

#[derive(Error, Debug)]
pub enum CodebaseIndexingError {
    #[error("Build tree error")]
    BuildTreeError,
    #[error("Repo size exceeded max file limit")]
    ExceededMaxFileLimit,
    #[error("Maximum directory depth exceeded")]
    MaxDepthExceeded,
    #[error("Failed to generate embeddings for some hashes")]
    FailedToGenerateEmbeddings,
    #[error("Failed to sync intermediate nodes")]
    FailedToSyncIntermediateNodes,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub struct CodebaseIndexStatus {
    has_pending: bool,
    has_synced_version: bool,
    last_sync_successful: Option<CodebaseIndexFinishedStatus>,
    sync_progress: Option<SyncProgress>,
}

impl CodebaseIndexStatus {
    pub fn has_pending(&self) -> bool {
        self.has_pending
    }

    pub fn has_synced_version(&self) -> bool {
        self.has_synced_version
    }

    pub fn last_sync_successful(&self) -> Option<bool> {
        self.last_sync_successful
            .as_ref()
            .map(|res| matches!(res, CodebaseIndexFinishedStatus::Completed))
    }

    pub fn last_sync_result(&self) -> Option<&CodebaseIndexFinishedStatus> {
        self.last_sync_successful.as_ref()
    }

    pub fn sync_progress(&self) -> Option<&SyncProgress> {
        self.sync_progress.as_ref()
    }
}

pub enum BuildSource<'a> {
    FromPath(&'a Path),
    FromPersistedMetadata(WorkspaceMetadata),
}

#[derive(Default)]
pub struct CodebaseIndexManager;

impl CodebaseIndexManager {
    pub fn new(
        _persisted_index_metadata: Vec<WorkspaceMetadata>,
        _max_index_count: Option<usize>,
        _max_files_repo_limit: usize,
        _embedding_generation_batch_size: usize,
        _store_client: Arc<dyn store_client::StoreClient>,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self
    }

    pub fn new_for_test(
        store_client: Arc<dyn store_client::StoreClient>,
        ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self::new(Vec::new(), None, usize::MAX, usize::MAX, store_client, ctx)
    }

    pub fn clean_up_deleted_indices(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn drop_index(&mut self, _root_path: PathBuf, ctx: &mut ModelContext<Self>) {
        ctx.emit(CodebaseIndexManagerEvent::SyncStateUpdated);
    }

    pub fn handle_active_session_changed(&mut self, _active_directory: &Path) {}

    pub fn update_max_limits(
        &mut self,
        _max_indices: Option<usize>,
        _max_files_repo_limit: usize,
        _embedding_generation_batch_size: usize,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn can_create_new_indices(&self) -> bool {
        false
    }

    pub fn handle_session_bootstrapped(&mut self, _working_directory: &Path) {}

    pub fn get_codebase_index_statuses<'a>(
        &'a self,
        _app: &'a AppContext,
    ) -> impl Iterator<Item = (&'a PathBuf, CodebaseIndexStatus)> {
        std::iter::empty()
    }

    pub fn get_codebase_index_status_for_path<'a>(
        &'a self,
        _root_path: &Path,
        _app: &'a AppContext,
    ) -> Option<CodebaseIndexStatus> {
        None
    }

    pub fn get_codebase_paths(&self) -> impl Iterator<Item = &PathBuf> {
        std::iter::empty()
    }

    pub fn num_active_indices(&self) -> usize {
        0
    }

    pub fn index_directory(&mut self, _directory: PathBuf, ctx: &mut ModelContext<Self>) {
        ctx.emit(CodebaseIndexManagerEvent::SyncStateUpdated);
    }

    pub fn build_and_sync_codebase_index(
        &mut self,
        _build_source: BuildSource,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn reset_codebase_indexing(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn root_path_for_codebase(&self, _path: &Path) -> Option<PathBuf> {
        None
    }

    pub fn try_manual_resync_codebase(&self, _repo_path: &Path, _ctx: &mut ModelContext<Self>) {}

    pub fn retrieve_relevant_files(
        &self,
        _query: String,
        _repo_path: &Path,
        _ctx: &mut ModelContext<Self>,
    ) -> Result<RetrievalID, RetrieveFileError> {
        Err(RetrieveFileError::IndexNotFound)
    }

    pub fn abort_retrieval_request(
        &self,
        _repo_path: &Path,
        _retrieval_id: RetrievalID,
        _ctx: &mut ModelContext<Self>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    pub fn write_snapshot(&mut self, _working_directory: &Path, _ctx: &mut ModelContext<Self>) {}

    pub fn trigger_incremental_sync_for_path(
        &mut self,
        _directory_path: &Path,
        _ctx: &mut ModelContext<Self>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

impl Entity for CodebaseIndexManager {
    type Event = CodebaseIndexManagerEvent;
}

impl SingletonEntity for CodebaseIndexManager {}
