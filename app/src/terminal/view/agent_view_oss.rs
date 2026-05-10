use warpui::{EntityId, ViewContext};

use crate::{
    ai::{
        agent::conversation::AIConversationId,
        blocklist::agent_view::{
            AgentViewEntryBlockParams, AgentViewEntryOrigin, EnterAgentViewError,
        },
    },
    terminal::view::{RichContentInsertionPosition, TerminalView},
};

pub const ENTER_AGAIN_TO_SEND_MESSAGE_ID: &str = "entry_message";

impl TerminalView {
    pub fn enter_agent_view(
        &mut self,
        _initial_prompt: Option<String>,
        _conversation_id: Option<AIConversationId>,
        _origin: AgentViewEntryOrigin,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn enter_agent_view_for_new_conversation(
        &mut self,
        _initial_prompt: Option<String>,
        _origin: AgentViewEntryOrigin,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn enter_agent_view_for_conversation(
        &mut self,
        _initial_prompt: Option<String>,
        _origin: AgentViewEntryOrigin,
        _conversation_id: AIConversationId,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn try_enter_agent_view(
        &mut self,
        _initial_prompt: Option<String>,
        _origin: AgentViewEntryOrigin,
        _conversation_id: Option<AIConversationId>,
        _ctx: &mut ViewContext<Self>,
    ) -> Result<AIConversationId, EnterAgentViewError> {
        Err(EnterAgentViewError::Unavailable)
    }

    pub(super) fn insert_agent_view_entry_block(
        &mut self,
        _params: AgentViewEntryBlockParams,
        _position: RichContentInsertionPosition,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn set_rich_content_agent_view_conversation_id(
        &mut self,
        _rich_content_view_id: EntityId,
        _conversation_id: AIConversationId,
    ) {
    }
}
