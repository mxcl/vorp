use crate::{
    code_review::diff_state::DiffMode, server::telemetry::CLIAgentType,
    view_components::find::FindDirection,
};
use std::fmt::Display;
use warp_core::telemetry::{EnablementState, TelemetryEvent, TelemetryEventDesc};

#[derive(Clone, Copy, Debug)]
pub enum GitButtonKind {
    Commit,
    Push,
    Publish,
    CreatePr,
    ViewPr,
}

#[derive(Clone, Copy, Debug)]
pub enum GitOperationKind {
    CommitOnly,
    CommitAndPush,
    CommitAndCreatePr,
    Push,
    Publish,
    CreatePr,
}

#[derive(Clone, Copy, Debug)]
pub enum GitDialogStatus {
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum CodeReviewPaneEntrypoint {
    GitDiffChip,
    AgentModeCompleted,
    AgentModeRunning,
    SlashCommand,
    InvokedByAgent,
    ForceOpened,
    CodeDiffHeader,
    PaneHeader,
    RightPanel,
    CLIAgentView,
    #[default]
    Other,
}

impl Display for CodeReviewPaneEntrypoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("disabled")
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AddToContextOrigin {
    SelectedText,
    Gutter,
    CodeReviewHeader,
}

#[derive(Clone, Copy, Debug)]
pub enum CodeReviewContextDestination {
    Pty,
    AgentInput,
    AgentAttachment,
    ActiveCommandBuffer,
    AgentReview,
    RichInput,
}

#[derive(Clone, Copy, Debug)]
pub enum DiffSetContextScope {
    All,
    File,
}

#[derive(Clone, Copy, Debug)]
pub enum PaneStateChange {
    Minimized,
    Maximized,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum CodeReviewTelemetryEvent {
    PaneOpened {
        entrypoint: CodeReviewPaneEntrypoint,
        is_code_mode_v2: bool,
        cli_agent: Option<CLIAgentType>,
    },
    AddToContext {
        origin: AddToContextOrigin,
        destination: CodeReviewContextDestination,
        diff_set_scope: Option<DiffSetContextScope>,
    },
    RevertHunkClicked,
    FileSaved,
    PaneStateChanged {
        state_change: PaneStateChange,
    },
    BaseChanged {
        mode: DiffMode,
    },
    CalculateDiffMetadataFailed {
        error: String,
    },
    LoadDiffFailed {
        error: String,
    },
    FindBarToggled {
        is_open: bool,
    },
    FindBarModeChanged {
        case_sensitive: bool,
        regex: bool,
    },
    FindNavigated {
        direction: FindDirection,
    },
    CommentEditorOpened,
    CommentAdded,
    CommentEdited,
    CommentDeleted {
        is_imported: bool,
    },
    CommentListExpanded {
        comment_count: usize,
    },
    ReviewSubmitted {
        comment_count: usize,
        file_count: usize,
        destination: CodeReviewContextDestination,
    },
    CommentListItemClicked,
    CommentRelocationFailed {
        fallback_count: usize,
    },
    CommentResolved {
        resolved_count: usize,
    },
    CommentsReceived {
        raw_count: usize,
        converted_count: usize,
        thread_count: usize,
    },
    CommentsAttached {
        active_count: usize,
        outdated_count: usize,
    },
    GitButtonTriggered {
        button: GitButtonKind,
    },
    GitDialogCompleted {
        operation: GitOperationKind,
        status: GitDialogStatus,
        error: Option<String>,
    },
}

impl TelemetryEvent for CodeReviewTelemetryEvent {
    fn name(&self) -> &'static str {
        "TelemetryDisabled"
    }

    fn payload(&self) -> Option<serde_json::Value> {
        None
    }

    fn description(&self) -> &'static str {
        ""
    }

    fn enablement_state(&self) -> EnablementState {
        EnablementState::Always
    }

    fn contains_ugc(&self) -> bool {
        false
    }

    fn event_descs() -> impl Iterator<Item = Box<dyn TelemetryEventDesc>> {
        std::iter::empty()
    }
}

warp_core::register_telemetry_event!(CodeReviewTelemetryEvent);
