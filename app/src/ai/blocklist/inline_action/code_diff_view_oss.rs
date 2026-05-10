use std::rc::Rc;

use ai::diff_validation::DiffType;
use warp_core::{HostId, platform::SessionPlatform};
use warp_util::file::FileSaveError;
use warpui::{
    AppContext, Element, Entity, ModelHandle, TypedActionView, View, ViewContext, elements::Empty,
};

use crate::{
    ai::{
        agent::{AIAgentActionId, AIIdentifiers, FileEdit, FileLocations},
        blocklist::{
            action_model::{BlocklistAIActionModel, RequestFileEditsFormatKind},
            block::{AIBlock, model::AIBlockModel},
        },
    },
    code::{DiffResult, diff_viewer::DisplayMode},
    menu::MenuItem,
    pane_group::{
        BackingView, PaneEvent,
        focus_state::PaneFocusHandle,
        pane::{PaneId, view},
    },
    terminal::ShellLaunchData,
};

pub fn init(_app: &mut AppContext) {}

#[derive(Debug, Clone)]
pub enum CodeDiffViewEvent {
    TryAccept,
    SavedAcceptedDiffs {
        diff: DiffResult,
        updated_files: Vec<(FileLocations, bool)>,
        file_contents: Vec<(String, String)>,
        deleted_files: Vec<String>,
        save_errors: Vec<Rc<FileSaveError>>,
    },
    Rejected,
    Pane(PaneEvent),
    EditModeChanged {
        enabled: bool,
    },
    DisplayModeChanged,
    CancelPassive,
    ViewDetails,
    ContinuePassiveCodeDiffWithAgent {
        accepted: bool,
    },
    LoadedDiffs,
}

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct DiffBase {
    pub content: String,
    pub file_path: String,
}

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct FileDiff {
    pub base: DiffBase,
    pub diff_type: DiffType,
}

impl FileDiff {
    pub fn new(content: String, file_path: String, diff_type: DiffType) -> FileDiff {
        FileDiff {
            base: DiffBase { content, file_path },
            diff_type,
        }
    }

    pub fn file_path(&self) -> String {
        self.base.file_path.clone()
    }
}

#[derive(Clone, Debug)]
pub struct SavingDiffs;

#[derive(Clone, Debug)]
pub enum CodeDiffState {
    Queued,
    WaitingForUser,
    Accepted(Option<SavingDiffs>),
    Rejected,
    Reverted,
    ViewOnly { is_complete: bool },
}

impl CodeDiffState {
    fn is_complete(&self) -> bool {
        matches!(
            self,
            CodeDiffState::Accepted(_)
                | CodeDiffState::Rejected
                | CodeDiffState::Reverted
                | CodeDiffState::ViewOnly { is_complete: true }
        )
    }
}

#[derive(Clone, Debug)]
pub enum CodeDiffViewAction {
    RevertChanges,
}

#[derive(Clone, Debug)]
pub enum DiffSessionType {
    Local,
    Remote(HostId),
}

#[derive(Clone)]
pub struct CodeDiffView {
    action_id: AIAgentActionId,
    state: CodeDiffState,
    diffs: Vec<FileDiff>,
    display_mode: DisplayMode,
    title: Option<String>,
    focus_handle: Option<PaneFocusHandle>,
    diff_session_type: DiffSessionType,
    original_pane_id: Option<PaneId>,
    is_passive: bool,
}

impl CodeDiffView {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        action_id: &AIAgentActionId,
        _model: &dyn AIBlockModel<View = AIBlock>,
        title: Option<String>,
        _identifiers: AIIdentifiers,
        _edit_format_kind: RequestFileEditsFormatKind,
        _should_show_speedbump: bool,
        _action_model: ModelHandle<BlocklistAIActionModel>,
        _session_platform: Option<SessionPlatform>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self::build(action_id, false, CodeDiffState::WaitingForUser, title)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_passive(
        action_id: &AIAgentActionId,
        title: Option<String>,
        _identifiers: AIIdentifiers,
        _edit_format_kind: RequestFileEditsFormatKind,
        _should_show_speedbump: bool,
        _session_platform: Option<SessionPlatform>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self::build(action_id, true, CodeDiffState::WaitingForUser, title)
    }

    fn build(
        action_id: &AIAgentActionId,
        is_passive: bool,
        state: CodeDiffState,
        title: Option<String>,
    ) -> Self {
        Self {
            action_id: action_id.clone(),
            state,
            diffs: Vec::new(),
            display_mode: DisplayMode::with_embedded(500.),
            title,
            focus_handle: None,
            diff_session_type: DiffSessionType::Local,
            original_pane_id: None,
            is_passive,
        }
    }

    pub fn set_diff_session_type(&mut self, session_type: DiffSessionType) {
        self.diff_session_type = session_type;
    }

    pub fn is_pending_diffs_empty(&self) -> bool {
        self.diffs.is_empty()
    }

    pub fn is_passive(&self) -> bool {
        self.is_passive
    }

    pub fn set_candidate_diffs(&mut self, diffs: Vec<FileDiff>, ctx: &mut ViewContext<Self>) {
        self.diffs = diffs;
        ctx.emit(CodeDiffViewEvent::LoadedDiffs);
        ctx.notify();
    }

    pub fn try_accept_action(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(CodeDiffViewEvent::TryAccept);
    }

    pub fn reject(&mut self, ctx: &mut ViewContext<Self>) {
        self.state = CodeDiffState::Rejected;
        if self.is_passive {
            ctx.emit(CodeDiffViewEvent::CancelPassive);
        } else {
            ctx.emit(CodeDiffViewEvent::Rejected);
        }
        ctx.notify();
    }

    pub fn expand_and_edit(&mut self, ctx: &mut ViewContext<Self>) {
        if self.display_mode.is_inline_banner() {
            self.set_embedded_display_mode(true, ctx);
        }
        ctx.emit(CodeDiffViewEvent::ViewDetails);
        ctx.emit(CodeDiffViewEvent::EditModeChanged { enabled: true });
        ctx.notify();
    }

    pub fn expand_inline_banner(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(CodeDiffViewEvent::DisplayModeChanged);
        ctx.notify();
    }

    pub fn dismiss(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(CodeDiffViewEvent::CancelPassive);
        ctx.notify();
    }

    pub fn accept_and_save(&mut self, ctx: &mut ViewContext<Self>) {
        self.state = CodeDiffState::Accepted(None);
        let updated_files = self
            .diffs
            .iter()
            .map(|diff| {
                (
                    FileLocations {
                        name: diff.file_path(),
                        lines: Vec::new(),
                    },
                    false,
                )
            })
            .collect();
        let file_contents = self
            .diffs
            .iter()
            .map(|diff| (diff.file_path(), diff.base.content.clone()))
            .collect();
        ctx.emit(CodeDiffViewEvent::SavedAcceptedDiffs {
            diff: DiffResult::default(),
            updated_files,
            file_contents,
            deleted_files: Vec::new(),
            save_errors: Vec::new(),
        });
        ctx.notify();
    }

    pub fn set_state(&mut self, state: CodeDiffState, ctx: &mut ViewContext<Self>) {
        self.state = state;
        ctx.notify();
    }

    pub fn state(&self) -> &CodeDiffState {
        &self.state
    }

    pub fn is_complete(&self) -> bool {
        self.state.is_complete()
    }

    pub fn set_embedded_display_mode(&mut self, embedded: bool, ctx: &mut ViewContext<Self>) {
        self.display_mode = if embedded {
            DisplayMode::with_embedded(500.)
        } else {
            DisplayMode::FullPane
        };
        ctx.emit(CodeDiffViewEvent::DisplayModeChanged);
        ctx.notify();
    }

    pub fn display_mode(&self) -> &DisplayMode {
        &self.display_mode
    }

    pub fn action_id(&self) -> &AIAgentActionId {
        &self.action_id
    }

    pub fn set_original_pane_id(&mut self, original_pane_id: Option<PaneId>) {
        self.original_pane_id = original_pane_id;
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn is_inline_banner_dismissed(&self) -> bool {
        false
    }

    pub fn selected_text(&self, _ctx: &AppContext) -> Option<String> {
        None
    }

    pub fn clear_all_selections(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn primary_file_path(&self, _app: &AppContext) -> Option<String> {
        self.diffs.first().map(FileDiff::file_path)
    }
}

impl Entity for CodeDiffView {
    type Event = CodeDiffViewEvent;
}

impl View for CodeDiffView {
    fn ui_name() -> &'static str {
        "CodeDiffView"
    }

    fn render(&self, _ctx: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for CodeDiffView {
    type Action = CodeDiffViewAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            CodeDiffViewAction::RevertChanges => {
                self.state = CodeDiffState::Reverted;
                ctx.notify();
            }
        }
    }
}

pub fn convert_file_edits_to_file_diffs(
    file_edits: Vec<FileEdit>,
    _shell_launch_data: &Option<ShellLaunchData>,
    _current_working_directory: &Option<String>,
) -> Vec<FileDiff> {
    file_edits
        .into_iter()
        .filter_map(|edit| {
            let path = edit.file()?.to_owned();
            let diff_type = match edit {
                FileEdit::Create { content, .. } => DiffType::creation(content.unwrap_or_default()),
                FileEdit::Delete { .. } => DiffType::deletion(0),
                FileEdit::Edit(_) => DiffType::update(Vec::new(), None),
            };
            Some(FileDiff::new(String::new(), path, diff_type))
        })
        .collect()
}

impl BackingView for CodeDiffView {
    type PaneHeaderOverflowMenuAction = CodeDiffViewAction;
    type CustomAction = ();
    type AssociatedData = ();

    fn pane_header_overflow_menu_items(
        &self,
        _ctx: &AppContext,
    ) -> Vec<MenuItem<CodeDiffViewAction>> {
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
        ctx.emit(CodeDiffViewEvent::Pane(PaneEvent::Close));
    }

    fn handle_custom_action(
        &mut self,
        _custom_action: &Self::CustomAction,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    fn focus_contents(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.focus_self();
    }

    fn render_header_content(
        &self,
        _ctx: &view::HeaderRenderContext<'_>,
        _app: &AppContext,
    ) -> view::HeaderContent {
        view::HeaderContent::simple("Requested Edit")
    }

    fn set_focus_handle(&mut self, focus_handle: PaneFocusHandle, _ctx: &mut ViewContext<Self>) {
        self.focus_handle = Some(focus_handle);
    }
}
