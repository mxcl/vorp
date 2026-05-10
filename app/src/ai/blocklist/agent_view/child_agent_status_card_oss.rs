use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, ModelHandle, TypedActionView, View, ViewContext};

use crate::ai::agent::conversation::AIConversationId;
use crate::ai::blocklist::agent_view::AgentViewController;

pub type ChildAgentStatusCard = StatusCard;
pub type ChildAgentStatusCardAction = StatusCardAction;

#[derive(Clone)]
pub enum StatusCardAction {
    Dismiss(AIConversationId),
}

impl std::fmt::Debug for StatusCardAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Action")
    }
}

pub struct StatusCard;

impl StatusCard {
    pub fn new(
        _agent_view_controller: ModelHandle<AgentViewController>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }
}

impl Entity for StatusCard {
    type Event = ();
}

impl TypedActionView for StatusCard {
    type Action = StatusCardAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for StatusCard {
    fn ui_name() -> &'static str {
        "StatusCard"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
