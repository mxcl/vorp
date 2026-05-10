use std::sync::Arc;

use parking_lot::FairMutex;
use warpui::prelude::Empty;
use warpui::{AppContext, Element, Entity, ModelHandle, TypedActionView, View, ViewContext};

use crate::ai::agent::conversation::AIConversationId;
use crate::ai::blocklist::agent_view::{AgentViewController, AgentViewEntryOrigin};
use crate::terminal::model::session::Sessions;
use crate::terminal::model_events::ModelEventDispatcher;
use crate::terminal::view::ambient_agent::AmbientAgentViewModel;
use crate::terminal::TerminalModel;

pub struct AgentViewZeroStateBlock;

impl AgentViewZeroStateBlock {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        _conversation_id: AIConversationId,
        _origin: AgentViewEntryOrigin,
        _agent_view_controller: ModelHandle<AgentViewController>,
        _sessions: &ModelHandle<Sessions>,
        _cloud_agent_view_model: Option<&ModelHandle<AmbientAgentViewModel>>,
        _terminal_model: Arc<FairMutex<TerminalModel>>,
        _model_events_dispatcher: &ModelHandle<ModelEventDispatcher>,
        _should_show_init_callout: bool,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }
}

impl View for AgentViewZeroStateBlock {
    fn ui_name() -> &'static str {
        "AgentViewZeroState"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

#[derive(Debug, Clone)]
pub enum AgentViewZeroStateEvent {
    ClickedInitCallout,
    OpenConversation { conversation_id: AIConversationId },
}

impl Entity for AgentViewZeroStateBlock {
    type Event = AgentViewZeroStateEvent;
}

#[derive(Debug, Clone)]
pub enum AgentViewZeroStateAction {
    ClickedInitCallout,
    ToggleOzUpdates,
    OpenConversation { conversation_id: AIConversationId },
}

impl TypedActionView for AgentViewZeroStateBlock {
    type Action = AgentViewZeroStateAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

pub fn render_ambient_credits_banner(_credits: i32, _app: &AppContext) -> Box<dyn Element> {
    Empty::new().finish()
}
