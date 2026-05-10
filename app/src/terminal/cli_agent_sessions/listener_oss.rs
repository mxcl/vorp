use warpui::{EntityId, ModelContext, ModelHandle};

use crate::terminal::model_events::ModelEventDispatcher;
use crate::terminal::CLIAgent;

pub fn agent_supports_rich_status(_agent: &CLIAgent) -> bool {
    false
}

pub fn is_agent_supported(_agent: &CLIAgent) -> bool {
    false
}

pub struct CLIAgentSessionListener;

impl warpui::Entity for CLIAgentSessionListener {
    type Event = ();
}

impl CLIAgentSessionListener {
    pub fn new(
        _terminal_view_id: EntityId,
        _agent: CLIAgent,
        _model_event_dispatcher: &ModelHandle<ModelEventDispatcher>,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self
    }
}
