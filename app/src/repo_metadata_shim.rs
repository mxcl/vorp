use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    future::Future,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    pin::Pin,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use thiserror::Error;
use warp_core::HostId;
use warp_util::standardized_path::StandardizedPath;
use warpui::{AppContext, Entity, ModelContext, ModelHandle, SingletonEntity};

pub mod ignore_compat {
    #[derive(Debug, Clone)]
    pub struct Gitignore;
}

#[derive(Error, Debug)]
pub enum RepoMetadataError {
    #[error("Repository not found: {0}")]
    RepoNotFound(String),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("Path encoding does not match local OS: {0}")]
    PathEncodingMismatch(StandardizedPath),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Build tree error: {0}")]
    BuildTree(BuildTreeError),
    #[error("Failed to start watcher: {0}")]
    WatcherError(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum BuildTreeError {
    #[error("Repo size exceeded max file limit")]
    ExceededMaxFileLimit,
    #[error("File is ignored")]
    Ignored,
    #[error("IO error reading path.")]
    IOError(#[from] std::io::Error),
    #[error("Symlink is not supported")]
    Symlink,
    #[error("Maximum directory depth exceeded")]
    MaxDepthExceeded,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct FileId(usize);

impl FileId {
    fn new() -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub path: StandardizedPath,
    pub file_id: FileId,
    pub extension: Option<String>,
    pub ignored: bool,
}

impl FileMetadata {
    pub fn new(path: PathBuf, ignored: bool) -> Self {
        let extension = path
            .extension()
            .and_then(|extension| extension.to_str())
            .map(str::to_string);
        Self {
            path: StandardizedPath::from_local_absolute_unchecked(&path),
            file_id: FileId::new(),
            extension,
            ignored,
        }
    }

    pub fn from_standardized(path: StandardizedPath, ignored: bool) -> Self {
        let extension = path.extension().map(str::to_string);
        Self {
            path,
            file_id: FileId::new(),
            extension,
            ignored,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DirectoryEntry {
    pub path: StandardizedPath,
    pub children: Vec<Entry>,
    pub ignored: bool,
    pub loaded: bool,
}

#[derive(Debug, Clone)]
pub enum Entry {
    File(FileMetadata),
    Directory(DirectoryEntry),
}

impl Entry {
    pub fn path(&self) -> &StandardizedPath {
        match self {
            Self::File(file) => &file.path,
            Self::Directory(directory) => &directory.path,
        }
    }

    pub fn loaded(&self) -> bool {
        match self {
            Self::File(_) => true,
            Self::Directory(directory) => directory.loaded,
        }
    }

    pub fn ignored(&self) -> bool {
        match self {
            Self::File(file) => file.ignored,
            Self::Directory(directory) => directory.ignored,
        }
    }
}

pub fn is_in_repo(_path: &str, _app: &AppContext) -> bool {
    false
}

pub fn gitignores_for_directory(_directory_path: &Path) -> Vec<ignore_compat::Gitignore> {
    Vec::new()
}

pub fn matches_gitignores(_path: &Path, _gitignores: &[ignore_compat::Gitignore]) -> bool {
    false
}

pub fn path_passes_filters(_path: &Path, _gitignores: &[ignore_compat::Gitignore]) -> bool {
    true
}

pub fn should_ignore_git_path(_path: &Path) -> bool {
    false
}

pub mod repository_identifier {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum RepositoryIdentifier {
        Local(StandardizedPath),
        Remote(RemoteRepositoryIdentifier),
    }

    impl RepositoryIdentifier {
        pub fn local(path: StandardizedPath) -> Self {
            Self::Local(path)
        }

        pub fn try_local(path: &Path) -> Option<Self> {
            StandardizedPath::try_from_local(path).ok().map(Self::Local)
        }

        pub fn local_path(&self) -> Option<&StandardizedPath> {
            match self {
                Self::Local(path) => Some(path),
                Self::Remote(_) => None,
            }
        }

        pub fn local_path_buf(&self) -> Option<PathBuf> {
            self.local_path().and_then(StandardizedPath::to_local_path)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct RemoteRepositoryIdentifier {
        pub host_id: HostId,
        pub path: StandardizedPath,
    }

    impl RemoteRepositoryIdentifier {
        pub fn new(host_id: HostId, path: StandardizedPath) -> Self {
            Self { host_id, path }
        }
    }

    impl From<RemoteRepositoryIdentifier> for RepositoryIdentifier {
        fn from(id: RemoteRepositoryIdentifier) -> Self {
            Self::Remote(id)
        }
    }
}

pub use repository_identifier::{RemoteRepositoryIdentifier, RepositoryIdentifier};

pub mod watcher {
    use super::*;

    #[derive(Default)]
    pub struct DirectoryWatcher;

    impl DirectoryWatcher {
        pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
            Self
        }

        pub fn get_watched_directory_for_path(
            &self,
            _path: &Path,
        ) -> Option<ModelHandle<Repository>> {
            None
        }

        pub fn is_directory_watched(&self, _path: &StandardizedPath) -> bool {
            false
        }

        pub fn add_directory(
            &mut self,
            _path: StandardizedPath,
            _ctx: &mut ModelContext<Self>,
        ) -> Result<ModelHandle<Repository>, RepoMetadataError> {
            Err(RepoMetadataError::RepoNotFound(
                "repository metadata is unavailable in this build".to_string(),
            ))
        }

        pub fn add_directory_with_git_dir(
            &mut self,
            _path: StandardizedPath,
            _external_git_directory: Option<StandardizedPath>,
            _ctx: &mut ModelContext<Self>,
        ) -> Result<ModelHandle<Repository>, RepoMetadataError> {
            Err(RepoMetadataError::RepoNotFound(
                "repository metadata is unavailable in this build".to_string(),
            ))
        }
    }

    impl Entity for DirectoryWatcher {
        type Event = ();
    }

    impl SingletonEntity for DirectoryWatcher {}

    #[derive(Debug, Clone)]
    pub struct TargetFile {
        pub path: PathBuf,
        pub is_ignored: bool,
    }

    impl TargetFile {
        pub fn new(path: PathBuf, is_ignored: bool) -> Self {
            Self { path, is_ignored }
        }
    }

    impl Hash for TargetFile {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.path.hash(state);
            self.is_ignored.hash(state);
        }
    }

    impl PartialEq for TargetFile {
        fn eq(&self, other: &Self) -> bool {
            self.path == other.path && self.is_ignored == other.is_ignored
        }
    }

    impl Eq for TargetFile {}

    impl PartialOrd for TargetFile {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for TargetFile {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.path
                .cmp(&other.path)
                .then_with(|| self.is_ignored.cmp(&other.is_ignored))
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct RepositoryUpdate {
        pub added: HashSet<TargetFile>,
        pub modified: HashSet<TargetFile>,
        pub deleted: HashSet<TargetFile>,
        pub moved: HashMap<TargetFile, TargetFile>,
        pub commit_updated: bool,
        pub index_lock_detected: bool,
    }

    impl RepositoryUpdate {
        pub fn is_empty(&self) -> bool {
            self.added.is_empty()
                && self.modified.is_empty()
                && self.deleted.is_empty()
                && self.moved.is_empty()
                && !self.commit_updated
                && !self.index_lock_detected
        }

        pub fn added_or_modified(&self) -> impl Iterator<Item = &TargetFile> {
            self.added.iter().chain(self.modified.iter())
        }

        pub fn into_added_or_modified(self) -> impl Iterator<Item = TargetFile> {
            self.added.into_iter().chain(self.modified)
        }

        pub fn contains_added_or_modified(&self, file: &TargetFile) -> bool {
            self.added.contains(file) || self.modified.contains(file)
        }
    }
}

pub use watcher::{DirectoryWatcher, RepositoryUpdate, TargetFile};

pub mod repository {
    use super::*;

    pub trait RepositorySubscriber: Send + Sync {
        fn on_scan(
            &mut self,
            repository: &Repository,
            ctx: &mut ModelContext<Repository>,
        ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

        fn on_files_updated(
            &mut self,
            repository: &Repository,
            update: &RepositoryUpdate,
            ctx: &mut ModelContext<Repository>,
        ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

        fn on_unsubscribe(&mut self, _ctx: &mut ModelContext<Repository>) {}
    }

    pub type SubscriberId = usize;

    pub struct StartWatching {
        pub subscriber_id: SubscriberId,
        pub registration_future:
            Pin<Box<dyn Future<Output = Result<(), RepoMetadataError>> + Send + 'static>>,
    }

    pub struct Repository {
        root_dir: StandardizedPath,
        external_git_directory: Option<StandardizedPath>,
    }

    impl Repository {
        pub fn root_dir(&self) -> &StandardizedPath {
            &self.root_dir
        }

        pub fn external_git_directory(&self) -> Option<&StandardizedPath> {
            self.external_git_directory.as_ref()
        }

        pub fn git_dir(&self) -> PathBuf {
            self.external_git_directory
                .as_ref()
                .and_then(StandardizedPath::to_local_path)
                .unwrap_or_else(|| self.root_dir.to_local_path_lossy().join(".git"))
        }

        pub fn common_git_dir(&self) -> PathBuf {
            self.git_dir()
        }

        pub fn watcher_count(&self) -> usize {
            0
        }

        pub fn start_watching(
            &mut self,
            _subscriber: Box<dyn RepositorySubscriber>,
            _ctx: &mut ModelContext<Self>,
        ) -> StartWatching {
            StartWatching {
                subscriber_id: 0,
                registration_future: Box::pin(async { Ok(()) }),
            }
        }

        pub fn stop_watching(
            &mut self,
            _subscriber_id: SubscriberId,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn check_gitignore_status(&self, _path: &Path) -> bool {
            false
        }
    }

    impl Entity for Repository {
        type Event = ();
    }

    pub struct BufferingRepositorySubscriber<S> {
        inner: S,
    }

    impl<S> BufferingRepositorySubscriber<S> {
        pub fn new(inner: S, _debounce: std::time::Duration) -> Self {
            Self { inner }
        }
    }

    impl<S: RepositorySubscriber> RepositorySubscriber for BufferingRepositorySubscriber<S> {
        fn on_scan(
            &mut self,
            repository: &Repository,
            ctx: &mut ModelContext<Repository>,
        ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
            self.inner.on_scan(repository, ctx)
        }

        fn on_files_updated(
            &mut self,
            repository: &Repository,
            update: &RepositoryUpdate,
            ctx: &mut ModelContext<Repository>,
        ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
            self.inner.on_files_updated(repository, update, ctx)
        }

        fn on_unsubscribe(&mut self, ctx: &mut ModelContext<Repository>) {
            self.inner.on_unsubscribe(ctx);
        }
    }
}

pub use repository::Repository;

pub mod repositories {
    use futures::future::ready;

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum RepoDetectionSource {
        TerminalNavigation,
        ProjectRulesIndexing,
        CodeReviewInitialization,
        CloudEnvironmentPrep,
    }

    pub enum DetectedRepositoriesEvent {
        DetectedGitRepo {
            repository: ModelHandle<Repository>,
            source: RepoDetectionSource,
        },
    }

    #[derive(Default)]
    pub struct DetectedRepositories {
        repository_roots: HashSet<PathBuf>,
    }

    impl DetectedRepositories {
        pub fn detect_possible_git_repo(
            &mut self,
            active_directory: &str,
            _source: RepoDetectionSource,
            _ctx: &mut ModelContext<Self>,
        ) -> impl Future<Output = Option<PathBuf>> {
            let root = find_git_repo(Path::new(active_directory));
            if let Some(root) = &root {
                self.repository_roots.insert(root.clone());
            }
            ready(root)
        }

        pub fn get_watched_repo_for_path(
            &self,
            _repo_path: &Path,
            _ctx: &AppContext,
        ) -> Option<ModelHandle<Repository>> {
            None
        }

        pub fn get_root_for_path(&self, path: &Path) -> Option<PathBuf> {
            self.repository_roots
                .iter()
                .find(|root| path.starts_with(root))
                .cloned()
                .or_else(|| find_git_repo(path))
        }

        pub fn insert_test_repo_root(&mut self, path: StandardizedPath) {
            if let Some(path) = path.to_local_path() {
                self.repository_roots.insert(path);
            }
        }
    }

    impl Entity for DetectedRepositories {
        type Event = DetectedRepositoriesEvent;
    }

    impl SingletonEntity for DetectedRepositories {}

    fn find_git_repo(path: &Path) -> Option<PathBuf> {
        let mut current = if path.is_file() { path.parent()? } else { path };
        loop {
            if current.join(".git").exists() {
                return Some(current.to_path_buf());
            }
            current = current.parent()?;
        }
    }
}

pub mod file_tree_store {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct FileTreeEntry {
        root_path: Arc<StandardizedPath>,
        state: HashMap<Arc<StandardizedPath>, FileTreeEntryState>,
    }

    impl FileTreeEntry {
        pub fn ignored(&self, path: &StandardizedPath) -> bool {
            self.get(path).is_some_and(FileTreeEntryState::ignored)
        }

        pub fn get(&self, path: &StandardizedPath) -> Option<&FileTreeEntryState> {
            self.state.get(path)
        }

        pub fn contains(&self, path: &StandardizedPath) -> bool {
            self.state.contains_key(path)
        }

        pub fn root_directory(&self) -> &Arc<StandardizedPath> {
            &self.root_path
        }

        pub fn child_paths(
            &self,
            _path: &StandardizedPath,
        ) -> impl Iterator<Item = &Arc<StandardizedPath>> {
            std::iter::empty()
        }

        pub fn get_mut(&mut self, path: &StandardizedPath) -> Option<&mut FileTreeEntryState> {
            self.state.get_mut(path)
        }
    }

    impl From<Entry> for FileTreeEntry {
        fn from(value: Entry) -> Self {
            let root_path = Arc::new(value.path().clone());
            let mut state = HashMap::new();
            match value {
                Entry::File(file) => {
                    state.insert(root_path.clone(), FileTreeEntryState::File(file.into()));
                }
                Entry::Directory(directory) => {
                    state.insert(
                        root_path.clone(),
                        FileTreeEntryState::Directory(FileTreeDirectoryEntryState {
                            path: root_path.clone(),
                            ignored: directory.ignored,
                            loaded: directory.loaded,
                        }),
                    );
                }
            }
            Self { root_path, state }
        }
    }

    #[derive(Debug, Clone)]
    pub enum FileTreeEntryState {
        File(FileTreeFileMetadata),
        Directory(FileTreeDirectoryEntryState),
    }

    impl FileTreeEntryState {
        pub fn set_ignored(&mut self, ignored: bool) {
            match self {
                Self::File(file) => file.ignored = ignored,
                Self::Directory(directory) => directory.ignored = ignored,
            }
        }

        pub fn ignored(&self) -> bool {
            match self {
                Self::File(file) => file.ignored,
                Self::Directory(directory) => directory.ignored,
            }
        }

        pub fn path(&self) -> &StandardizedPath {
            match self {
                Self::File(file) => &file.path,
                Self::Directory(directory) => &directory.path,
            }
        }

        pub fn loaded(&self) -> bool {
            match self {
                Self::File(_) => true,
                Self::Directory(directory) => directory.loaded,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct FileTreeFileMetadata {
        pub path: Arc<StandardizedPath>,
        pub file_id: FileId,
        pub extension: Option<String>,
        pub ignored: bool,
    }

    impl From<FileMetadata> for FileTreeFileMetadata {
        fn from(value: FileMetadata) -> Self {
            Self {
                path: Arc::new(value.path),
                file_id: value.file_id,
                extension: value.extension,
                ignored: value.ignored,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct FileTreeDirectoryEntryState {
        pub path: Arc<StandardizedPath>,
        pub ignored: bool,
        pub loaded: bool,
    }

    #[derive(Debug, Clone)]
    pub struct FileTreeState {
        pub entry: FileTreeEntry,
        pub gitignores: Vec<ignore_compat::Gitignore>,
    }

    impl FileTreeState {
        pub fn new(
            entry: Entry,
            gitignores: Vec<ignore_compat::Gitignore>,
            _repository: Option<ModelHandle<Repository>>,
        ) -> Self {
            Self {
                entry: entry.into(),
                gitignores,
            }
        }

        pub fn new_lazy_loaded(entry: Entry) -> Self {
            Self {
                entry: entry.into(),
                gitignores: Vec::new(),
            }
        }

        pub fn from_file_tree_entry(entry: FileTreeEntry) -> Self {
            Self {
                entry,
                gitignores: Vec::new(),
            }
        }
    }
}

pub use file_tree_store::{
    FileTreeDirectoryEntryState, FileTreeEntry, FileTreeEntryState, FileTreeFileMetadata,
    FileTreeState,
};

pub mod local_model {
    use super::*;

    #[derive(Debug)]
    pub enum IndexedRepoState {
        Pending,
        Indexed(FileTreeState),
        Failed(RepoMetadataError),
    }

    #[derive(Debug)]
    pub enum RepositoryMetadataEvent {
        RepositoryUpdated { path: StandardizedPath },
        RepositoryRemoved { path: StandardizedPath },
        FileTreeUpdated { paths: Vec<StandardizedPath> },
        FileTreeEntryUpdated { path: StandardizedPath },
        UpdatingRepositoryFailed { path: StandardizedPath },
        IncrementalUpdateReady { update: RepoMetadataUpdate },
    }

    pub enum RepoContent<'a> {
        File(&'a FileTreeFileMetadata),
        Directory(&'a FileTreeDirectoryEntryState),
    }

    type RepoContentFilter = dyn for<'a> Fn(&RepoContent<'a>) -> bool + Send + Sync;

    pub struct GetContentsArgs {
        pub include_folders: bool,
        pub include_ignored: bool,
        pub filter: Option<Arc<RepoContentFilter>>,
    }

    impl Default for GetContentsArgs {
        fn default() -> Self {
            Self {
                include_folders: true,
                include_ignored: false,
                filter: None,
            }
        }
    }

    impl GetContentsArgs {
        pub fn include_ignored(mut self) -> Self {
            self.include_ignored = true;
            self
        }

        pub fn exclude_folders(mut self) -> Self {
            self.include_folders = false;
            self
        }

        pub fn with_filter<F>(self, filter: F) -> Self
        where
            F: for<'a> Fn(&RepoContent<'a>) -> bool + Send + Sync + 'static,
        {
            Self {
                include_folders: self.include_folders,
                include_ignored: self.include_ignored,
                filter: Some(Arc::new(filter)),
            }
        }
    }

    pub struct LocalRepoMetadataModel;

    impl LocalRepoMetadataModel {
        pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
            Self
        }

        pub fn set_emit_incremental_updates(&mut self, _enabled: bool) {}
    }

    impl Entity for LocalRepoMetadataModel {
        type Event = RepositoryMetadataEvent;
    }
}

pub use local_model::{LocalRepoMetadataModel, RepoContent};

pub mod remote_model {
    use super::*;

    pub enum RemoteRepositoryMetadataEvent {
        RepositoryUpdated { id: RemoteRepositoryIdentifier },
        RepositoryRemoved { id: RemoteRepositoryIdentifier },
        FileTreeEntryUpdated { id: RemoteRepositoryIdentifier },
    }

    pub struct RemoteRepoMetadataModel;

    impl RemoteRepoMetadataModel {
        pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
            Self
        }
    }

    impl Entity for RemoteRepoMetadataModel {
        type Event = RemoteRepositoryMetadataEvent;
    }
}

pub use remote_model::RemoteRepoMetadataModel;

pub mod file_tree_update {
    use super::*;

    #[derive(Debug, Clone, Default)]
    pub struct RepoMetadataUpdate {
        pub entries: Vec<FileTreeEntryUpdate>,
        pub removed_paths: Vec<StandardizedPath>,
    }

    #[derive(Debug, Clone)]
    pub struct FileTreeEntryUpdate {
        pub path: StandardizedPath,
        pub metadata: RepoNodeMetadata,
    }

    #[derive(Debug, Clone)]
    pub enum RepoNodeMetadata {
        Directory(DirectoryNodeMetadata),
        File(FileNodeMetadata),
    }

    #[derive(Debug, Clone)]
    pub struct DirectoryNodeMetadata {
        pub ignored: bool,
    }

    #[derive(Debug, Clone)]
    pub struct FileNodeMetadata {
        pub ignored: bool,
        pub extension: Option<String>,
    }
}

pub use file_tree_update::RepoMetadataUpdate;

pub mod wrapper_model {
    use super::*;

    #[derive(Debug)]
    pub enum RepoMetadataEvent {
        RepositoryUpdated { id: RepositoryIdentifier },
        RepositoryRemoved { id: RepositoryIdentifier },
        FileTreeUpdated { ids: Vec<RepositoryIdentifier> },
        FileTreeEntryUpdated { id: RepositoryIdentifier },
        UpdatingRepositoryFailed { id: RepositoryIdentifier },
        IncrementalUpdateReady { update: RepoMetadataUpdate },
    }

    pub struct RepoMetadataModel;

    impl RepoMetadataModel {
        pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
            Self
        }

        pub fn new_with_incremental_updates(_ctx: &mut ModelContext<Self>) -> Self {
            Self
        }

        pub fn get_repository(
            &self,
            _id: &RepositoryIdentifier,
            _ctx: &AppContext,
        ) -> Option<&FileTreeState> {
            None
        }

        pub fn has_repository(&self, _id: &RepositoryIdentifier, _ctx: &AppContext) -> bool {
            false
        }

        pub fn repository_state(
            &self,
            _id: &RepositoryIdentifier,
            _ctx: &AppContext,
        ) -> Option<&local_model::IndexedRepoState> {
            None
        }

        pub fn get_repo_contents(
            &self,
            _id: &RepositoryIdentifier,
            _args: local_model::GetContentsArgs,
            _ctx: &AppContext,
        ) -> Option<Vec<RepoContent<'_>>> {
            None
        }

        pub fn find_repository_for_path(
            &self,
            _path: &Path,
            _ctx: &AppContext,
        ) -> Option<RepositoryIdentifier> {
            None
        }

        pub fn index_directory(
            &self,
            _repo_path: StandardizedPath,
            _ctx: &mut ModelContext<Self>,
        ) -> impl Future<Output = Result<(), RepoMetadataError>> {
            futures::future::ready(Err(RepoMetadataError::RepoNotFound(
                "repository metadata is unavailable in this build".to_string(),
            )))
        }

        pub fn index_lazy_loaded_path(
            &self,
            _path: StandardizedPath,
            _ctx: &mut ModelContext<Self>,
        ) -> impl Future<Output = Result<(), RepoMetadataError>> {
            futures::future::ready(Err(RepoMetadataError::RepoNotFound(
                "repository metadata is unavailable in this build".to_string(),
            )))
        }

        pub fn load_directory(
            &self,
            _id: &RepositoryIdentifier,
            _dir_path: StandardizedPath,
            _ctx: &mut ModelContext<Self>,
        ) -> impl Future<Output = Result<(), RepoMetadataError>> {
            futures::future::ready(Err(RepoMetadataError::RepoNotFound(
                "repository metadata is unavailable in this build".to_string(),
            )))
        }

        pub fn remove_lazy_loaded_path(
            &self,
            _path: &StandardizedPath,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn insert_remote_snapshot(
            &self,
            _host_id: HostId,
            _update: &RepoMetadataUpdate,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn apply_remote_incremental_update(
            &self,
            _host_id: &HostId,
            _update: &RepoMetadataUpdate,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn remove_remote_repositories_for_host(
            &self,
            _host_id: &HostId,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn remove_repository(
            &mut self,
            _id: &RepositoryIdentifier,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn remote_repository_ids(
            &self,
            _ctx: &AppContext,
        ) -> impl Iterator<Item = &RemoteRepositoryIdentifier> {
            std::iter::empty()
        }

        pub fn is_lazy_loaded_path(&self, _path: &StandardizedPath, _ctx: &AppContext) -> bool {
            false
        }
    }

    impl Entity for RepoMetadataModel {
        type Event = RepoMetadataEvent;
    }

    impl SingletonEntity for RepoMetadataModel {}
}

pub use wrapper_model::{RepoMetadataEvent, RepoMetadataModel};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanonicalizedPath(PathBuf);

impl std::fmt::Display for CanonicalizedPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.display().fmt(f)
    }
}

impl CanonicalizedPath {
    pub fn as_path_buf(&self) -> &PathBuf {
        &self.0
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }

    pub fn pop(&mut self) -> bool {
        self.0.pop()
    }

    pub fn starts_with(&self, path: &CanonicalizedPath) -> bool {
        self.0.starts_with(&path.0)
    }
}

impl TryFrom<PathBuf> for CanonicalizedPath {
    type Error = std::io::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Ok(Self(dunce::canonicalize(value)?))
    }
}

impl TryFrom<&Path> for CanonicalizedPath {
    type Error = std::io::Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        Ok(Self(dunce::canonicalize(value)?))
    }
}

impl TryFrom<&PathBuf> for CanonicalizedPath {
    type Error = std::io::Error;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        Ok(Self(dunce::canonicalize(value)?))
    }
}

impl TryFrom<&str> for CanonicalizedPath {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(dunce::canonicalize(value)?))
    }
}

impl From<CanonicalizedPath> for PathBuf {
    fn from(canonical: CanonicalizedPath) -> Self {
        canonical.0
    }
}

impl From<&CanonicalizedPath> for PathBuf {
    fn from(canonical: &CanonicalizedPath) -> Self {
        canonical.0.clone()
    }
}

impl Borrow<Path> for CanonicalizedPath {
    fn borrow(&self) -> &Path {
        self.0.borrow()
    }
}

impl Borrow<PathBuf> for CanonicalizedPath {
    fn borrow(&self) -> &PathBuf {
        &self.0
    }
}

impl From<CanonicalizedPath> for StandardizedPath {
    fn from(canonical: CanonicalizedPath) -> Self {
        StandardizedPath::try_from_local(canonical.as_path())
            .expect("CanonicalizedPath is always absolute")
    }
}
