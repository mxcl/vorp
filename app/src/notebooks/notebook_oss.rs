use std::sync::Arc;

use crate::{
    ai::document::ai_document_model::AIDocumentId,
    cloud_object::Space,
    drive::{items::WarpDriveItemId, CloudObjectTypeAndId},
    pane_group::{
        focus_state::PaneFocusHandle,
        pane::view::{HeaderContent, HeaderRenderContext},
        BackingView, PaneConfiguration, PaneEvent,
    },
    server::{ids::SyncId, telemetry::SharingDialogSource},
    workflows::{WorkflowSource, WorkflowType},
};
use warpui::{
    elements::Empty, AppContext, Element, Entity, ModelHandle, TypedActionView, View, ViewContext,
};

#[derive(Debug, Clone, PartialEq)]
pub enum NotebookEvent {
    RunWorkflow {
        workflow: Arc<WorkflowType>,
        source: WorkflowSource,
    },
    EditWorkflow(SyncId),
    ViewInWarpDrive(WarpDriveItemId),
    Pane(PaneEvent),
    MoveToSpace {
        cloud_object_type_and_id: CloudObjectTypeAndId,
        new_space: Space,
    },
    OpenDriveObjectShareDialog {
        cloud_object_type_and_id: CloudObjectTypeAndId,
        invitee_email: Option<String>,
        source: SharingDialogSource,
    },
    AttachPlanAsContext(AIDocumentId),
}

impl From<PaneEvent> for NotebookEvent {
    fn from(event: PaneEvent) -> Self {
        NotebookEvent::Pane(event)
    }
}

pub struct NotebookView {
    pane_configuration: ModelHandle<PaneConfiguration>,
}

impl NotebookView {
    pub fn new(ctx: &mut ViewContext<Self>) -> Self {
        Self {
            pane_configuration: ctx.add_model(|_| PaneConfiguration::new("")),
        }
    }

    pub fn pane_configuration(&self) -> ModelHandle<PaneConfiguration> {
        self.pane_configuration.clone()
    }

    pub fn notebook_id(&self, _ctx: &impl warpui::ModelAsRef) -> Option<SyncId> {
        None
    }

    pub fn on_detach(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn focus(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.focus_self();
    }

    pub fn notebook_link(&self, _ctx: &AppContext) -> Option<String> {
        None
    }

    pub fn selected_text(&self, _ctx: &AppContext) -> Option<String> {
        None
    }

    pub fn is_plan(&self, _ctx: &AppContext) -> bool {
        false
    }
}

impl Entity for NotebookView {
    type Event = NotebookEvent;
}

impl View for NotebookView {
    fn ui_name() -> &'static str {
        "NotebookView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for NotebookView {
    type Action = ();

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl BackingView for NotebookView {
    type PaneHeaderOverflowMenuAction = ();
    type CustomAction = ();
    type AssociatedData = ();

    fn handle_pane_header_overflow_menu_action(
        &mut self,
        _action: &Self::PaneHeaderOverflowMenuAction,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    fn close(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(NotebookEvent::Pane(PaneEvent::Close));
    }

    fn focus_contents(&mut self, ctx: &mut ViewContext<Self>) {
        self.focus(ctx);
    }

    fn render_header_content(
        &self,
        _ctx: &HeaderRenderContext<'_>,
        app: &AppContext,
    ) -> HeaderContent {
        HeaderContent::simple(self.pane_configuration.as_ref(app).title())
    }

    fn set_focus_handle(&mut self, _focus_handle: PaneFocusHandle, _ctx: &mut ViewContext<Self>) {}
}
