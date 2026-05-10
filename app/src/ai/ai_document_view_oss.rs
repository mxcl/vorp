#[cfg(feature = "local_fs")]
use crate::code::editor_management::CodeSource;
#[cfg(feature = "local_fs")]
use crate::util::file::external_editor::settings::EditorLayout;
#[cfg(feature = "local_fs")]
use crate::util::openable_file_type::FileTarget;
use crate::{
    ai::document::ai_document_model::{AIDocumentId, AIDocumentVersion},
    drive::items::WarpDriveItemId,
    menu::MenuItem,
    pane_group::{
        focus_state::PaneFocusHandle, pane::view, BackingView, PaneConfiguration, PaneEvent,
    },
    terminal::view::TerminalView,
};
#[cfg(feature = "local_fs")]
use warp_util::path::LineAndColumnArg;
use warpui::{
    elements::Empty, AppContext, Element, Entity, ModelHandle, TypedActionView, View, ViewContext,
    ViewHandle, WindowId,
};

pub const DEFAULT_PLANNING_DOCUMENT_TITLE: &str = "";

pub fn init(_app: &mut AppContext) {}

#[derive(Clone, PartialEq)]
pub enum AIDocumentAction {
    Close,
    SelectVersion(AIDocumentVersion),
    Export,
    OpenVersionMenu,
    CreateWarpDriveNotebook,
    RevertToDocumentVersion,
    SendUpdatedPlan,
    CopyLink(String),
    CopyPlanId,
    ShowInWarpDrive,
    AttachToActiveSession,
}

impl std::fmt::Debug for AIDocumentAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DocumentAction")
    }
}

#[derive(Clone)]
pub enum AIDocumentEvent {
    Pane(PaneEvent),
    CloseRequested,
    ViewInWarpDrive(WarpDriveItemId),
    #[cfg(feature = "local_fs")]
    OpenCodeInWarp {
        source: CodeSource,
        layout: EditorLayout,
        line_col: Option<LineAndColumnArg>,
    },
    #[cfg(feature = "local_fs")]
    OpenFileWithTarget {
        path: std::path::PathBuf,
        target: FileTarget,
        line_col: Option<LineAndColumnArg>,
    },
    AttachPlanAsContext(AIDocumentId),
}

impl std::fmt::Debug for AIDocumentEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DocumentEvent")
    }
}

impl From<PaneEvent> for AIDocumentEvent {
    fn from(event: PaneEvent) -> Self {
        Self::Pane(event)
    }
}

pub struct AIDocumentView {
    document_id: AIDocumentId,
    document_version: AIDocumentVersion,
    pane_configuration: ModelHandle<PaneConfiguration>,
    focus_handle: Option<PaneFocusHandle>,
    original_terminal_view: Option<ViewHandle<TerminalView>>,
}

impl AIDocumentView {
    pub fn new(
        document_id: AIDocumentId,
        document_version: AIDocumentVersion,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            document_id,
            document_version,
            pane_configuration: ctx.add_model(|_ctx| PaneConfiguration::new("")),
            focus_handle: None,
            original_terminal_view: None,
        }
    }

    pub fn pane_configuration(&self) -> &ModelHandle<PaneConfiguration> {
        &self.pane_configuration
    }

    pub fn document_id(&self) -> &AIDocumentId {
        &self.document_id
    }

    pub fn document_version(&self) -> AIDocumentVersion {
        self.document_version
    }

    pub fn selected_text(&self, _ctx: &AppContext) -> Option<String> {
        None
    }

    pub fn set_original_terminal_view(&mut self, terminal_view: Option<ViewHandle<TerminalView>>) {
        self.original_terminal_view = terminal_view;
    }

    pub fn terminal_view(&self) -> Option<ViewHandle<TerminalView>> {
        self.original_terminal_view.clone()
    }

    pub fn focus(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.focus_self();
        ctx.emit(AIDocumentEvent::Pane(PaneEvent::FocusSelf));
    }

    pub fn bind_window(&self, _window_id: WindowId, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for AIDocumentView {
    type Event = AIDocumentEvent;
}

impl View for AIDocumentView {
    fn ui_name() -> &'static str {
        "DocumentView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for AIDocumentView {
    type Action = AIDocumentAction;

    fn handle_action(&mut self, action: &AIDocumentAction, ctx: &mut ViewContext<Self>) {
        match action {
            AIDocumentAction::Close => self.close(ctx),
            AIDocumentAction::SelectVersion(version) => {
                self.document_version = *version;
                ctx.notify();
            }
            AIDocumentAction::AttachToActiveSession => {
                ctx.emit(AIDocumentEvent::AttachPlanAsContext(self.document_id));
            }
            AIDocumentAction::Export
            | AIDocumentAction::OpenVersionMenu
            | AIDocumentAction::CreateWarpDriveNotebook
            | AIDocumentAction::RevertToDocumentVersion
            | AIDocumentAction::SendUpdatedPlan
            | AIDocumentAction::CopyLink(_)
            | AIDocumentAction::CopyPlanId
            | AIDocumentAction::ShowInWarpDrive => {}
        }
    }
}

impl BackingView for AIDocumentView {
    type PaneHeaderOverflowMenuAction = AIDocumentAction;
    type CustomAction = AIDocumentAction;
    type AssociatedData = ();

    fn handle_pane_header_overflow_menu_action(
        &mut self,
        action: &Self::PaneHeaderOverflowMenuAction,
        ctx: &mut ViewContext<Self>,
    ) {
        self.handle_action(action, ctx);
    }

    fn handle_custom_action(&mut self, action: &Self::CustomAction, ctx: &mut ViewContext<Self>) {
        self.handle_action(action, ctx);
    }

    fn close(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(AIDocumentEvent::Pane(PaneEvent::Close));
    }

    fn focus_contents(&mut self, ctx: &mut ViewContext<Self>) {
        self.focus(ctx);
    }

    fn set_focus_handle(&mut self, focus_handle: PaneFocusHandle, _ctx: &mut ViewContext<Self>) {
        self.focus_handle = Some(focus_handle);
    }

    fn pane_header_overflow_menu_items(
        &self,
        _ctx: &AppContext,
    ) -> Vec<MenuItem<Self::PaneHeaderOverflowMenuAction>> {
        Vec::new()
    }

    fn render_header_content(
        &self,
        _header_ctx: &view::HeaderRenderContext<'_>,
        _app: &AppContext,
    ) -> view::HeaderContent {
        view::HeaderContent::simple(DEFAULT_PLANNING_DOCUMENT_TITLE)
    }
}
