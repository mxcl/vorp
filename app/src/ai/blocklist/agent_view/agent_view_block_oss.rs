use pathfinder_color::ColorU;
use warp_core::ui::appearance::Appearance;
use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, ModelHandle, TypedActionView, View, ViewContext};

use crate::ai::agent::conversation::AIConversationId;
use crate::ai::blocklist::agent_view::{AgentViewController, AgentViewEntryOrigin};

pub struct AgentViewEntryBlockParams {
    pub conversation_id: AIConversationId,
    pub is_new: bool,
    pub is_restored: bool,
    pub origin: AgentViewEntryOrigin,
    pub agent_view_controller: ModelHandle<AgentViewController>,
}

pub type AgentViewEntryBlock = EntryBlock;
pub type EnterAgentBlockAction = EntryBlockAction;
pub type AgentViewEntryBlockEvent = EntryBlockEvent;

pub struct EntryBlock {
    conversation_id: AIConversationId,
}

impl EntryBlock {
    pub fn new(params: AgentViewEntryBlockParams, _ctx: &mut ViewContext<Self>) -> Self {
        let _ = (
            params.is_new,
            params.is_restored,
            params.origin,
            params.agent_view_controller,
        );
        Self {
            conversation_id: params.conversation_id,
        }
    }
}

pub fn render_block_container(
    _origin: AgentViewEntryOrigin,
    content: Box<dyn Element>,
    _background: ColorU,
    _appearance: &Appearance,
    _are_block_dividers_enabled: bool,
) -> Box<dyn Element> {
    content
}

#[derive(Clone)]
pub enum EntryBlockEvent {
    EnterAgentView { conversation_id: AIConversationId },
}

impl std::fmt::Debug for EntryBlockEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Event")
    }
}

impl Entity for EntryBlock {
    type Event = EntryBlockEvent;
}

#[derive(Clone)]
pub enum EntryBlockAction {
    EnterAgentMode { conversation_id: AIConversationId },
}

impl std::fmt::Debug for EntryBlockAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Action")
    }
}

impl TypedActionView for EntryBlock {
    type Action = EntryBlockAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            EntryBlockAction::EnterAgentMode { conversation_id } => {
                ctx.emit(EntryBlockEvent::EnterAgentView {
                    conversation_id: *conversation_id,
                });
            }
        }
    }
}

impl View for EntryBlock {
    fn ui_name() -> &'static str {
        "EntryBlock"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        let _ = self.conversation_id;
        Empty::new().finish()
    }
}
