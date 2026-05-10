use std::path::{Path, PathBuf};

use anyhow::Result;
use warpui::{Entity, ModelContext, ModelHandle, SingletonEntity};

use super::diff_state::DiffStats;

#[derive(Debug, Clone)]
pub struct GitStatusMetadata {
    pub current_branch_name: String,
    pub main_branch_name: String,
    pub stats_against_head: DiffStats,
}

pub struct GitStatusUpdateModel;

impl GitStatusUpdateModel {
    pub fn new() -> Self {
        Self
    }

    pub fn subscribe(
        &mut self,
        _repo_path: &Path,
        _ctx: &mut ModelContext<Self>,
    ) -> Result<ModelHandle<GitRepoStatusModel>> {
        anyhow::bail!("git status updates are not available in this build")
    }
}

impl Entity for GitStatusUpdateModel {
    type Event = ();
}

impl SingletonEntity for GitStatusUpdateModel {}

pub struct GitRepoStatusModel {
    repo_path: PathBuf,
    metadata: Option<GitStatusMetadata>,
}

#[derive(Debug)]
pub enum GitRepoStatusEvent {
    MetadataChanged,
}

impl Entity for GitRepoStatusModel {
    type Event = GitRepoStatusEvent;
}

impl GitRepoStatusModel {
    pub fn metadata(&self) -> Option<&GitStatusMetadata> {
        self.metadata.as_ref()
    }

    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    pub fn refresh_metadata(&mut self, _ctx: &mut ModelContext<Self>) {}
}
