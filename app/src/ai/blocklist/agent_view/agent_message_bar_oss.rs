use std::{marker::PhantomData, sync::Arc};

use parking_lot::FairMutex;
use warpui::elements::{Element, Empty, MouseStateHandle};
use warpui::{AppContext, Entity, ModelHandle, View, ViewContext};

use crate::ai::agent::conversation::AIConversation;
use crate::ai::agent::AIAgentExchangeId;
use crate::ai::blocklist::agent_view::{
    shortcuts::AgentShortcutViewModel, AgentViewController, EphemeralMessageModel,
};
use crate::ai::blocklist::{BlocklistAIContextModel, BlocklistAIInputModel};
use crate::terminal::input::buffer_model::InputBufferModel;
use crate::terminal::input::slash_command_model::SlashCommandModel;
use crate::terminal::input::suggestions_mode_model::InputSuggestionsModeModel;
use crate::terminal::model::TerminalModel;

#[derive(Clone, Default)]
pub struct AgentMessageBarMouseStates {
    pub clear_attached_context: MouseStateHandle,
}

#[derive(Copy, Clone)]
pub struct AgentMessageArgs<'a> {
    _marker: PhantomData<&'a ()>,
}

pub type AgentMessageBar = MessageBar;

pub struct MessageBar;

pub(crate) fn fork_from_last_known_good_state_exchange_id(
    _active_conversation: &AIConversation,
    _terminal_model: &TerminalModel,
) -> Option<AIAgentExchangeId> {
    None
}

impl Entity for MessageBar {
    type Event = ();
}

impl MessageBar {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        _agent_view_controller: ModelHandle<AgentViewController>,
        _ephemeral_message_model: ModelHandle<EphemeralMessageModel>,
        _shortcut_view_model: ModelHandle<AgentShortcutViewModel>,
        _input_buffer_model: ModelHandle<InputBufferModel>,
        _input_model: ModelHandle<BlocklistAIInputModel>,
        _input_suggestions_model: ModelHandle<InputSuggestionsModeModel>,
        _slash_command_model: ModelHandle<SlashCommandModel>,
        _context_model: ModelHandle<BlocklistAIContextModel>,
        _terminal_model: Arc<FairMutex<TerminalModel>>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }
}

impl View for MessageBar {
    fn ui_name() -> &'static str {
        "MessageBar"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
