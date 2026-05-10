use warpui::elements::{ClippedScrollStateHandle, Element, Empty, MouseStateHandle};
use warpui::{AppContext, Entity, ModelHandle, TypedActionView, View, ViewContext};

use crate::ai::agent::conversation::AIConversationId;
use crate::ai::blocklist::agent_view::AgentViewController;

pub type OrchestrationPillBar = PillBar;
pub type OrchestrationPillBarAction = PillBarAction;

#[derive(Clone, PartialEq, Eq)]
pub enum PillBarAction {
    OpenMenu(AIConversationId),
    CloseMenu,
    OpenInNewPane(AIConversationId),
    OpenInNewTab(AIConversationId),
    Stop(AIConversationId),
    Kill(AIConversationId),
    SetHoveredPill(Option<AIConversationId>),
    FocusOpenedConversation(AIConversationId),
}

impl std::fmt::Debug for PillBarAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Action")
    }
}

pub struct PillBar;

impl PillBar {
    pub fn new(
        _agent_view_controller: ModelHandle<AgentViewController>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }
}

impl Entity for PillBar {
    type Event = ();
}

impl TypedActionView for PillBar {
    type Action = PillBarAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for PillBar {
    fn ui_name() -> &'static str {
        "PillBar"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

pub fn render_orchestration_breadcrumbs(
    _agent_view_controller: &AgentViewController,
    _parent_crumb_mouse_state: MouseStateHandle,
    _horizontal_scroll_state: ClippedScrollStateHandle,
    _app: &AppContext,
) -> Option<Box<dyn Element>> {
    None
}
