use std::sync::Arc;

use warpui::prelude::Empty;
use warpui::{AppContext, Element, Entity, TypedActionView, View, ViewContext};

use crate::notebooks::NotebookId;
use crate::view_components::action_button::ActionButtonTheme;

use super::Artifact;

/// OSS builds keep artifact data compatibility but do not render AI artifact UI.
pub struct ArtifactButtonsRow;

impl ArtifactButtonsRow {
    pub fn new(_artifacts: &[Artifact], _ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn with_theme(
        _artifacts: &[Artifact],
        _theme: Arc<dyn ActionButtonTheme>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn update_artifacts(&mut self, _artifacts: &[Artifact], _ctx: &mut ViewContext<Self>) {}

    pub fn is_empty(&self) -> bool {
        true
    }
}

pub enum ArtifactButtonsRowEvent {
    OpenPlan { notebook_uid: NotebookId },
    CopyBranch { branch: String },
    OpenPullRequest { url: String },
    ViewScreenshots { artifact_uids: Vec<String> },
    DownloadFile { artifact_uid: String },
}

#[derive(Debug, Clone)]
pub enum ArtifactButtonAction {
    OpenPlan { notebook_uid: NotebookId },
    CopyBranch { branch: String },
    OpenPullRequest { url: String },
    ViewScreenshots { artifact_uids: Vec<String> },
    DownloadFile { artifact_uid: String },
}

impl Entity for ArtifactButtonsRow {
    type Event = ArtifactButtonsRowEvent;
}

impl View for ArtifactButtonsRow {
    fn ui_name() -> &'static str {
        "View"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for ArtifactButtonsRow {
    type Action = ArtifactButtonAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}
