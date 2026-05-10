use std::collections::HashMap;

use crate::{
    ai::execution_profiles::profiles::ClientProfileId,
    pane_group::{
        focus_state::PaneFocusHandle, pane::view, BackingView, ExecutionProfileEditorPane,
        PaneConfiguration, PaneContent, PaneEvent,
    },
    PaneViewLocator,
};
use warpui::{
    elements::Empty, AppContext, Element, Entity, EntityId, ModelContext, ModelHandle,
    SingletonEntity, TypedActionView, View, ViewContext, WindowId,
};

pub const HEADER_TEXT: &str = "Unavailable";

#[derive(Debug, Clone)]
pub enum ExecutionProfileEditorViewEvent {
    Pane(PaneEvent),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionProfileEditorViewAction {
    Save,
    Close,
    DeleteProfile,
}

pub struct ExecutionProfileEditorView {
    profile_id: ClientProfileId,
    pane_configuration: ModelHandle<PaneConfiguration>,
    focus_handle: Option<PaneFocusHandle>,
}

impl ExecutionProfileEditorView {
    pub fn new(profile_id: ClientProfileId, ctx: &mut ViewContext<Self>) -> Self {
        Self {
            profile_id,
            pane_configuration: ctx.add_model(|_ctx| PaneConfiguration::new(HEADER_TEXT)),
            focus_handle: None,
        }
    }

    pub fn profile_id(&self) -> ClientProfileId {
        self.profile_id
    }

    pub fn pane_configuration(&self) -> ModelHandle<PaneConfiguration> {
        self.pane_configuration.clone()
    }

    pub fn focus(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.focus_self();
    }
}

impl View for ExecutionProfileEditorView {
    fn ui_name() -> &'static str {
        "ExecutionProfileEditorView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl Entity for ExecutionProfileEditorView {
    type Event = ExecutionProfileEditorViewEvent;
}

impl TypedActionView for ExecutionProfileEditorView {
    type Action = ExecutionProfileEditorViewAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            ExecutionProfileEditorViewAction::Close
            | ExecutionProfileEditorViewAction::DeleteProfile => {
                ctx.emit(ExecutionProfileEditorViewEvent::Pane(PaneEvent::Close));
            }
            _ => {}
        }
    }
}

impl BackingView for ExecutionProfileEditorView {
    type PaneHeaderOverflowMenuAction = ExecutionProfileEditorViewAction;
    type CustomAction = ();
    type AssociatedData = ();

    fn handle_pane_header_overflow_menu_action(
        &mut self,
        action: &Self::PaneHeaderOverflowMenuAction,
        ctx: &mut ViewContext<Self>,
    ) {
        self.handle_action(action, ctx);
    }

    fn close(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(ExecutionProfileEditorViewEvent::Pane(PaneEvent::Close));
    }

    fn focus_contents(&mut self, ctx: &mut ViewContext<Self>) {
        self.focus(ctx);
    }

    fn render_header_content(
        &self,
        _ctx: &view::HeaderRenderContext<'_>,
        _app: &AppContext,
    ) -> view::HeaderContent {
        view::HeaderContent::simple(HEADER_TEXT)
    }

    fn set_focus_handle(&mut self, focus_handle: PaneFocusHandle, _ctx: &mut ViewContext<Self>) {
        self.focus_handle = Some(focus_handle);
    }
}

#[derive(Default)]
pub struct ExecutionProfileEditorManager {
    panes: HashMap<WindowId, HashMap<ClientProfileId, ExecutionProfileEditorPaneData>>,
}

#[derive(Clone, Copy)]
struct ExecutionProfileEditorPaneData {
    locator: PaneViewLocator,
}

impl ExecutionProfileEditorManager {
    pub fn find_pane(
        &self,
        window_id: WindowId,
        profile_id: ClientProfileId,
    ) -> Option<PaneViewLocator> {
        self.panes
            .get(&window_id)
            .and_then(|m| m.get(&profile_id))
            .map(|d| d.locator)
    }

    pub fn register_pane(
        &mut self,
        pane: &ExecutionProfileEditorPane,
        pane_group_id: EntityId,
        window_id: WindowId,
        profile_id: ClientProfileId,
        _ctx: &mut ModelContext<Self>,
    ) {
        let locator = PaneViewLocator {
            pane_group_id,
            pane_id: pane.id(),
        };
        self.panes
            .entry(window_id)
            .or_default()
            .insert(profile_id, ExecutionProfileEditorPaneData { locator });
    }

    pub fn deregister_pane(&mut self, window_id: &WindowId, profile_id: &ClientProfileId) {
        if let Some(map) = self.panes.get_mut(window_id) {
            map.remove(profile_id);
            if map.is_empty() {
                self.panes.remove(window_id);
            }
        }
    }
}

impl Entity for ExecutionProfileEditorManager {
    type Event = ();
}

impl SingletonEntity for ExecutionProfileEditorManager {}
