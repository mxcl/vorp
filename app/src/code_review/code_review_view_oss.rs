use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

use warp_util::{
    file::{FileLoadError, FileSaveError},
    path::LineAndColumnArg,
};
use warpui::{
    AppContext, Element, Entity, EventContext, ModelHandle, TypedActionView, View, ViewContext,
    elements::{Empty, MouseStateHandle},
};

use crate::{
    ai::agent::AgentReviewCommentBatch,
    code_review::{
        DiffSetScope,
        comments::{CommentId, ReviewCommentBatch},
        diff_state::{DiffMode, DiffStateModel, DiffStats},
    },
    menu::MenuItem,
    pane_group::{
        BackingView, PaneEvent, PaneId,
        focus_state::PaneFocusHandle,
        pane::view::{HeaderContent, HeaderRenderContext},
    },
    terminal::view::TerminalView,
    workspace::view::right_panel::{ReviewDestination, ReviewSubmissionResult},
};

#[cfg(feature = "local_fs")]
use crate::util::openable_file_type::FileTarget;

pub(crate) const CONTENT_LEFT_MARGIN: f32 = 16.;
pub(crate) const CONTENT_RIGHT_MARGIN: f32 = 4.;
pub const CODE_REVIEW_TOOLTIP_TEXT: &str = "";

pub fn render_file_navigation_button<F>(
    _appearance: &crate::appearance::Appearance,
    _is_sidebar_expanded: bool,
    _mouse_state: MouseStateHandle,
    _on_click: F,
) -> Box<dyn Element>
where
    F: Fn(&mut EventContext<'_>) + 'static,
{
    Empty::new().finish()
}

#[derive(Clone, PartialEq)]
pub enum CodeReviewAction {
    OpenInNewTab {
        path: PathBuf,
        line_and_column: Option<LineAndColumnArg>,
    },
    ToggleFileExpanded(PathBuf),
    OpenHeaderMenu,
    SetDiffMode(DiffMode),
    ToggleFileSidebar,
    FileSelected(usize),
    ToggleMaximize,
    SaveAllUnsavedFiles,
    SaveAllFiles {
        paths: Vec<PathBuf>,
    },
    RefreshGitState,
    UndoRevert,
    Close,
    EmitPaneEvent(PaneEvent),
    ShowDiscardConfirmDialog(Option<PathBuf>),
    ConfirmDiscardFile,
    CancelDiscardFile,
    ToggleStashChanges,
    ToggleFileSelection(PathBuf),
    AddDiffSetAsContext(DiffSetScope),
    CopyFilePath(PathBuf),
    OpenCommentComposerFromHeader,
    ShowFindBar,
    FocusView,
    InitProjectForCurrentDirectory,
    OpenRepository,
    OpenCommitDialog,
    ToggleGitOperationsMenu,
    OpenPushDialog,
    OpenCreatePrDialog,
    ViewPr(String),
    PublishBranch,
}

impl std::fmt::Debug for CodeReviewAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ReviewAction")
    }
}

#[derive(Clone)]
pub enum CodeReviewViewEvent {
    Pane(PaneEvent),
    FileEdited {
        path: PathBuf,
    },
    FileSaved {
        path: PathBuf,
    },
    FileLoadError {
        path: PathBuf,
        error: Rc<FileLoadError>,
    },
    FileSaveError {
        path: PathBuf,
        error: Rc<FileSaveError>,
    },
    #[cfg(feature = "local_fs")]
    OpenFileWithTarget {
        path: PathBuf,
        target: FileTarget,
        line_col: Option<LineAndColumnArg>,
    },
    ReviewSubmitted,
    SubmitReviewComments {
        comments: AgentReviewCommentBatch,
        repo_path: PathBuf,
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

impl std::fmt::Debug for CodeReviewViewEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ReviewEvent")
    }
}

#[derive(Clone, PartialEq)]
pub struct CommentListDebugState {
    pub review_destination: ReviewDestination,
    pub total_comments: usize,
    pub sendable_comments: usize,
    pub is_collapsed: bool,
    pub is_outdated_section_collapsed: Option<bool>,
    pub ai_available: bool,
    pub ai_enabled: bool,
    pub send_button_tooltip_text: String,
}

impl std::fmt::Debug for CommentListDebugState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CommentListDebugState")
    }
}

#[derive(Clone, PartialEq)]
pub struct CodeReviewCommentDebugState {
    pub repo_path: Option<PathBuf>,
    pub has_active_comment_model: bool,
    pub comment_list: CommentListDebugState,
}

impl std::fmt::Debug for CodeReviewCommentDebugState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ReviewCommentDebugState")
    }
}

pub struct CodeReviewView {
    repo_path: Option<PathBuf>,
    diff_state_model: ModelHandle<DiffStateModel>,
    focus_handle: Option<PaneFocusHandle>,
    file_sidebar_expanded: bool,
    containing_pane_id: Option<PaneId>,
    terminal_view: Option<warpui::WeakViewHandle<TerminalView>>,
}

impl CodeReviewView {
    pub fn new(
        repo_path: Option<PathBuf>,
        diff_state_model: ModelHandle<DiffStateModel>,
        _comment_batch_model: Option<ModelHandle<ReviewCommentBatch>>,
        terminal_view: Option<warpui::WeakViewHandle<TerminalView>>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            repo_path,
            diff_state_model,
            focus_handle: None,
            file_sidebar_expanded: false,
            containing_pane_id: None,
            terminal_view,
        }
    }

    pub fn repo_path(&self) -> Option<&PathBuf> {
        self.repo_path.as_ref()
    }

    pub fn diff_state_model(&self) -> &ModelHandle<DiffStateModel> {
        &self.diff_state_model
    }

    pub fn update_current_repo(&mut self, repo_path: Option<PathBuf>, ctx: &mut ViewContext<Self>) {
        self.repo_path = repo_path;
        ctx.notify();
    }

    pub fn on_open(&mut self, repo_path: Option<PathBuf>, ctx: &mut ViewContext<Self>) {
        self.update_current_repo(repo_path, ctx);
    }

    pub fn on_close(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn set_terminal_view(&mut self, terminal_view: warpui::WeakViewHandle<TerminalView>) {
        self.terminal_view = Some(terminal_view);
    }

    pub fn set_review_destination(
        &mut self,
        _destination: ReviewDestination,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn debug_review_comment_state(&self, _ctx: &AppContext) -> CodeReviewCommentDebugState {
        CodeReviewCommentDebugState {
            repo_path: self.repo_path.clone(),
            has_active_comment_model: false,
            comment_list: CommentListDebugState {
                review_destination: ReviewDestination::None,
                total_comments: 0,
                sendable_comments: 0,
                is_collapsed: true,
                is_outdated_section_collapsed: None,
                ai_available: false,
                ai_enabled: false,
                send_button_tooltip_text: String::new(),
            },
        }
    }

    pub fn handle_review_submission_result(
        &mut self,
        _result: ReviewSubmissionResult,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn handle_maximization_toggle(&mut self, is_maximized: bool, ctx: &mut ViewContext<Self>) {
        self.file_sidebar_expanded = is_maximized;
        ctx.notify();
    }

    pub fn has_unsaved_changes(&self, _ctx: &AppContext) -> bool {
        false
    }

    pub fn set_pane_id(&mut self, pane_id: PaneId) {
        self.containing_pane_id = Some(pane_id);
    }

    pub fn file_sidebar_expanded(&self) -> bool {
        self.file_sidebar_expanded
    }

    pub fn has_file_states(&self) -> bool {
        false
    }

    pub fn loaded_diff_stats(&self) -> Option<DiffStats> {
        None
    }

    pub fn open_file_in_tab(
        &self,
        _path: &Path,
        _line_and_column: Option<LineAndColumnArg>,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(crate) fn expand_comment_list(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub(crate) fn navigate_to_imported_comment(
        &mut self,
        _comment_id: CommentId,
        diff_mode: DiffMode,
        ctx: &mut ViewContext<Self>,
    ) {
        self.set_diff_base(diff_mode, ctx);
    }

    pub(crate) fn set_diff_base(&mut self, diff_mode: DiffMode, ctx: &mut ViewContext<Self>) {
        self.diff_state_model.update(ctx, |model, ctx| {
            model.set_diff_mode_and_fetch_base(diff_mode, ctx);
        });
    }

    pub fn render_loading_state(_appearance: &crate::appearance::Appearance) -> Box<dyn Element> {
        Empty::new().finish()
    }

    pub fn render_remote_state(
        _appearance: &crate::appearance::Appearance,
        _open_repo_button: Option<Box<dyn Element>>,
    ) -> Box<dyn Element> {
        Empty::new().finish()
    }

    pub fn render_wsl_state(
        _appearance: &crate::appearance::Appearance,
        _open_repo_button: Option<Box<dyn Element>>,
    ) -> Box<dyn Element> {
        Empty::new().finish()
    }

    pub fn render_not_repo_state(
        _appearance: &crate::appearance::Appearance,
        _open_repo_button: Option<Box<dyn Element>>,
    ) -> Box<dyn Element> {
        Empty::new().finish()
    }

    pub fn render_diff_stats(
        _stats: &DiffStats,
        _appearance: &crate::appearance::Appearance,
    ) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl Entity for CodeReviewView {
    type Event = CodeReviewViewEvent;
}

impl View for CodeReviewView {
    fn ui_name() -> &'static str {
        "ReviewView"
    }

    fn render(&self, _ctx: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for CodeReviewView {
    type Action = CodeReviewAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            CodeReviewAction::Close => ctx.emit(CodeReviewViewEvent::Pane(PaneEvent::Close)),
            CodeReviewAction::ToggleMaximize => {
                ctx.emit(CodeReviewViewEvent::Pane(PaneEvent::ToggleMaximized))
            }
            CodeReviewAction::EmitPaneEvent(event) => {
                ctx.emit(CodeReviewViewEvent::Pane(event.clone()));
            }
            CodeReviewAction::SetDiffMode(mode) => self.set_diff_base(mode.clone(), ctx),
            CodeReviewAction::FocusView => ctx.focus_self(),
            _ => {}
        }
    }
}

impl BackingView for CodeReviewView {
    type PaneHeaderOverflowMenuAction = CodeReviewAction;
    type CustomAction = CodeReviewAction;
    type AssociatedData = ();

    fn pane_header_overflow_menu_items(
        &self,
        _ctx: &AppContext,
    ) -> Vec<MenuItem<CodeReviewAction>> {
        vec![]
    }

    fn handle_pane_header_overflow_menu_action(
        &mut self,
        action: &Self::PaneHeaderOverflowMenuAction,
        ctx: &mut ViewContext<Self>,
    ) {
        self.handle_action(action, ctx);
    }

    fn close(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(CodeReviewViewEvent::Pane(PaneEvent::Close));
    }

    fn focus_contents(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.focus_self();
    }

    fn handle_custom_action(&mut self, action: &Self::CustomAction, ctx: &mut ViewContext<Self>) {
        self.handle_action(action, ctx);
    }

    fn render_header_content(
        &self,
        _ctx: &HeaderRenderContext<'_>,
        _app: &AppContext,
    ) -> HeaderContent {
        HeaderContent::simple("")
    }

    fn set_focus_handle(&mut self, focus_handle: PaneFocusHandle, _ctx: &mut ViewContext<Self>) {
        self.focus_handle = Some(focus_handle);
    }
}
