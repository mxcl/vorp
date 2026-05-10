use std::path::PathBuf;

#[cfg(feature = "local_fs")]
use crate::util::openable_file_type::FileTarget;
use crate::{
    code_review::{diff_state::DiffStateModel, telemetry_event::CodeReviewContextDestination},
    pane_group::{PaneGroup, WorkingDirectoriesModel},
    terminal::{view::TerminalView, CLIAgent},
};
use warp_util::path::LineAndColumnArg;
use warpui::{
    elements::Empty, AppContext, Element, Entity, ModelHandle, TypedActionView, View, ViewContext,
    ViewHandle, WeakViewHandle,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ReviewDestination {
    None,
    Warp,
    Cli(CLIAgent),
}

pub enum ReviewSubmissionResult {
    Success {
        comment_count: usize,
        file_count: usize,
        destination: CodeReviewContextDestination,
    },
    Error,
}

#[derive(Clone, Debug)]
pub enum RightPanelAction {
    ToggleFileSidebar,
    SelectRepo {
        repo_path: PathBuf,
        from_dropdown: bool,
    },
    OpenRepository,
    ToggleMaximize,
}

#[derive(Clone, Debug)]
pub enum RightPanelEvent {
    ToggleMaximize,
    #[cfg(feature = "local_fs")]
    OpenFileWithTarget {
        path: PathBuf,
        target: FileTarget,
        line_col: Option<LineAndColumnArg>,
    },
    OpenFileInNewTab {
        path: PathBuf,
        line_and_column: Option<LineAndColumnArg>,
    },
    #[cfg(not(target_family = "wasm"))]
    OpenLspLogs {
        log_path: PathBuf,
    },
}

pub struct RightPanelView {
    pub active_pane_group: Option<ViewHandle<PaneGroup>>,
    is_maximized: bool,
    panel_position: super::PanelPosition,
}

impl RightPanelView {
    pub fn init(_app: &mut AppContext) {}

    pub fn new(
        _working_directories_model: ModelHandle<WorkingDirectoriesModel>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            active_pane_group: None,
            is_maximized: false,
            panel_position: super::PanelPosition::Right,
        }
    }

    pub fn set_agent_management_view_open(&mut self, _is_open: bool, _ctx: &mut ViewContext<Self>) {
    }

    pub fn set_panel_position(
        &mut self,
        position: super::PanelPosition,
        ctx: &mut ViewContext<Self>,
    ) {
        self.panel_position = position;
        ctx.notify();
    }

    #[cfg(feature = "local_fs")]
    pub fn update_session_env(
        &mut self,
        _is_remote: bool,
        _is_wsl: bool,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn selected_repo_path(&self) -> Option<&PathBuf> {
        None
    }

    #[cfg(feature = "local_fs")]
    pub fn update_selected_repo(&mut self, _repo_path: PathBuf, _ctx: &mut ViewContext<Self>) {}

    pub fn set_active_pane_group(
        &mut self,
        pane_group: ViewHandle<PaneGroup>,
        _working_directories_model: &ModelHandle<WorkingDirectoriesModel>,
        ctx: &mut ViewContext<Self>,
    ) {
        self.active_pane_group = Some(pane_group);
        ctx.notify();
    }

    pub fn open_code_review(
        &mut self,
        _repo_path: Option<PathBuf>,
        _diff_state_model: ModelHandle<DiffStateModel>,
        _terminal_view: WeakViewHandle<TerminalView>,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn close_code_review(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn set_maximized(&mut self, is_maximized: bool, ctx: &mut ViewContext<Self>) {
        self.is_maximized = is_maximized;
        ctx.notify();
    }

    pub fn focus_active_code_review_view(&self, _ctx: &mut ViewContext<Self>) {}

    pub fn log_review_comment_send_status_for_active_tab(&self, _ctx: &AppContext) {}

    pub fn recompute_terminal_availability(&self, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for RightPanelView {
    type Event = RightPanelEvent;
}

impl TypedActionView for RightPanelView {
    type Action = RightPanelAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for RightPanelView {
    fn ui_name() -> &'static str {
        "RightPanelView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        let _ = (self.is_maximized, self.panel_position);
        Empty::new().finish()
    }
}
