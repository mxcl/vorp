use std::path::{Path, PathBuf};

use crate::{
    ai::{agent::AIAgentActionId, skills::SkillOpenOrigin},
    code::view::CodeView,
    code_review::code_review_view::CodeReviewView,
    pane_group::{PaneGroup, PaneId},
    workspace::PaneViewLocator,
};
use ai::skills::SkillReference;
use serde::{Deserialize, Serialize};
use warp_util::path::LineAndColumnArg;
use warpui::{AppContext, Entity, EntityId, ModelContext, SingletonEntity, ViewHandle, WindowId};

pub struct CodeEditorSummary<'a> {
    pub unsaved_changes: Vec<&'a CodeEditorStatus>,
}

impl<'a> CodeEditorSummary<'a> {
    pub fn new(editors: &'a [CodeEditorStatus]) -> Self {
        let unsaved_changes = editors
            .iter()
            .filter(|editor| editor.unsaved_changes)
            .collect();

        Self { unsaved_changes }
    }
}

#[derive(Copy, Clone)]
pub struct CodeEditorStatus {
    unsaved_changes: bool,
}

impl CodeEditorStatus {
    pub fn new(unsaved_changes: bool) -> Self {
        Self { unsaved_changes }
    }

    pub fn all_editors(_app: &AppContext) -> impl Iterator<Item = Self> + '_ {
        std::iter::empty()
    }

    pub fn editors_in_window(
        _window_id: WindowId,
        _app: &AppContext,
    ) -> impl Iterator<Item = Self> + '_ {
        std::iter::empty()
    }

    pub fn editors_in_tab<'a>(
        _tab: &ViewHandle<PaneGroup>,
        _app: &'a AppContext,
    ) -> impl Iterator<Item = Self> + 'a {
        std::iter::empty()
    }

    pub fn editor_status(_editor: &ViewHandle<CodeView>, _app: &AppContext) -> Self {
        Self::new(false)
    }

    pub fn status_for_code_review(_review: &ViewHandle<CodeReviewView>, _app: &AppContext) -> Self {
        Self::new(false)
    }

    pub fn code_review_views_in_window(
        _window_id: WindowId,
        _app: &AppContext,
    ) -> impl Iterator<Item = Self> + '_ {
        std::iter::empty()
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum CodeSource {
    New {
        default_directory: Option<PathBuf>,
    },
    Link {
        path: PathBuf,
        range_start: Option<LineAndColumnArg>,
        range_end: Option<LineAndColumnArg>,
    },
    AIAction {
        id: AIAgentActionId,
    },
    ProjectRules {
        path: PathBuf,
    },
    FileTree {
        path: PathBuf,
    },
    Finder {
        path: PathBuf,
    },
    Skill {
        reference: SkillReference,
        path: PathBuf,
        origin: SkillOpenOrigin,
    },
}

impl CodeSource {
    pub fn default_directory(&self) -> Option<&PathBuf> {
        match self {
            Self::New { default_directory } => default_directory.as_ref(),
            Self::Link { .. }
            | Self::AIAction { .. }
            | Self::ProjectRules { .. }
            | Self::FileTree { .. }
            | Self::Finder { .. }
            | Self::Skill { .. } => None,
        }
    }

    pub fn path(&self) -> Option<PathBuf> {
        match self {
            Self::New { .. } | Self::AIAction { .. } => None,
            Self::Link { path, .. }
            | Self::ProjectRules { path }
            | Self::FileTree { path }
            | Self::Finder { path }
            | Self::Skill { path, .. } => Some(path.clone()),
        }
    }

    pub fn is_bundled_skill(&self) -> bool {
        matches!(
            self,
            Self::Skill {
                reference: SkillReference::BundledSkillId(_),
                ..
            }
        )
    }

    pub fn omit_line_col(&self) -> CodeSource {
        if let CodeSource::Link { path, .. } = self {
            CodeSource::Link {
                path: path.clone(),
                range_start: None,
                range_end: None,
            }
        } else {
            self.clone()
        }
    }

    pub fn telemetry_source_name(&self) -> &'static str {
        match self {
            Self::New { .. } => "new",
            Self::Link { .. } => "link",
            Self::AIAction { .. } => "ai_action",
            Self::ProjectRules { .. } => "project_rules",
            Self::FileTree { .. } => "file_tree",
            Self::Finder { .. } => "finder",
            Self::Skill { .. } => "skill",
        }
    }

    pub fn is_restorable(&self) -> bool {
        !matches!(self, Self::AIAction { .. })
    }
}

pub enum CodeManagerEvent {
    EditCompleted { action_id: AIAgentActionId },
}

#[derive(Default)]
pub struct CodeManager;

impl CodeManager {
    pub fn register_pane(
        &mut self,
        _pane_group_id: EntityId,
        _window_id: WindowId,
        _pane_id: PaneId,
        _source: CodeSource,
    ) {
    }

    pub fn deregister_pane(&mut self, _source: &CodeSource) {}

    pub fn get_locator_for_path_in_tab(
        &self,
        _pane_group_id: EntityId,
        _path: &Path,
    ) -> Option<PaneViewLocator> {
        None
    }

    pub fn complete_pending_diffs(&mut self, source: CodeSource, ctx: &mut ModelContext<Self>) {
        if let CodeSource::AIAction { id } = source {
            ctx.emit(CodeManagerEvent::EditCompleted { action_id: id });
        }
    }
}

impl Entity for CodeManager {
    type Event = CodeManagerEvent;
}

impl SingletonEntity for CodeManager {}
