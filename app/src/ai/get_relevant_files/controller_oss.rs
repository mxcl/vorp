use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
};

use ai::index::locations::CodeContextLocation;
use warpui::{AppContext, Entity, ModelContext};

use crate::ai::agent::AIAgentActionId;

#[derive(Debug)]
pub enum GetRelevantFilesControllerEvent {
    Success {
        action_id: AIAgentActionId,
        fragments: Arc<HashSet<CodeContextLocation>>,
    },
    Error {
        action_id: AIAgentActionId,
    },
}

impl GetRelevantFilesControllerEvent {
    pub fn action_id(&self) -> &AIAgentActionId {
        match self {
            GetRelevantFilesControllerEvent::Success { action_id, .. } => action_id,
            GetRelevantFilesControllerEvent::Error { action_id } => action_id,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetRelevantFilesError {
    #[error("Repo outline is still being computed.")]
    Pending,
    #[error("Failed to create outline.")]
    CreateFailed,
    #[error("Relevant file search is disabled in OSS builds.")]
    Missing,
}

#[derive(Default)]
pub struct GetRelevantFilesController;

impl GetRelevantFilesController {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }

    pub fn send_request(
        &mut self,
        _directory: &Path,
        _query: String,
        _partial_path_segments: Option<&Vec<String>>,
        _action_id: AIAgentActionId,
        _ctx: &mut ModelContext<Self>,
    ) -> Result<(), GetRelevantFilesError> {
        Err(GetRelevantFilesError::Missing)
    }

    pub fn root_directory_for_search(
        &self,
        _directory: &Path,
        _app: &AppContext,
    ) -> Option<PathBuf> {
        None
    }

    pub fn cancel_request_for_action(
        &mut self,
        _action_id: &AIAgentActionId,
        _ctx: &mut ModelContext<Self>,
    ) {
    }
}

impl Entity for GetRelevantFilesController {
    type Event = GetRelevantFilesControllerEvent;
}
