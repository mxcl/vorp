use std::time::Duration;

use uuid::Uuid;
use warpui::elements::{Element, Empty, Icon};
use warpui::{AppContext, Entity, EntityId, TypedActionView, View, ViewContext, WindowId};

pub struct AgentToastStack {
    latest_toast_navigation_data: Option<(WindowId, usize, EntityId)>,
}

impl AgentToastStack {
    pub fn new(_timeout: Duration, _ctx: &mut ViewContext<Self>) -> Self {
        Self {
            latest_toast_navigation_data: None,
        }
    }

    pub fn add_toast(&mut self, toast: AgentToast, _ctx: &mut ViewContext<Self>) {
        self.latest_toast_navigation_data =
            Some((toast.window_id, toast.tab_index, toast.terminal_view_id));
    }

    pub fn dismiss_toast_by_uuid(&mut self, _uuid: &Uuid, _ctx: &mut ViewContext<Self>) {}

    pub fn cancel_dismissal_timeout(&mut self, _uuid: &Uuid) {}

    pub fn start_dismissal_timeout(&mut self, _uuid: Uuid, _ctx: &mut ViewContext<Self>) {}

    pub fn latest_toast_uuid(&self) -> Option<Uuid> {
        None
    }

    pub fn get_latest_toast_navigation_data(&self) -> Option<(WindowId, usize, EntityId)> {
        self.latest_toast_navigation_data
    }
}

impl Entity for AgentToastStack {
    type Event = ();
}

#[derive(Debug)]
pub enum AgentToastAction {
    ClickDismissButton(Uuid),
    CancelDismissalTimeout(Uuid),
    StartDismissalTimeout(Uuid),
    ClickToastBody(Uuid),
}

impl TypedActionView for AgentToastStack {
    type Action = AgentToastAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for AgentToastStack {
    fn ui_name() -> &'static str {
        "AgentToastStack"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

#[derive(Clone)]
pub struct AgentToast {
    window_id: WindowId,
    tab_index: usize,
    terminal_view_id: EntityId,
}

impl AgentToast {
    pub fn new(
        _task_name: String,
        _icon: Icon,
        window_id: WindowId,
        tab_index: usize,
        terminal_view_id: EntityId,
    ) -> Self {
        Self {
            window_id,
            tab_index,
            terminal_view_id,
        }
    }
}
