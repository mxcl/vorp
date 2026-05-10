use chrono::{DateTime, Local};
use warp_cli::agent::Harness;
use warpui::{elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext};

use crate::{
    ai::{
        agent::conversation::{AIConversation, AIConversationId, ConversationStatus},
        ambient_agents::AmbientAgentTaskId,
        artifacts::Artifact,
    },
    notebooks::NotebookId,
    server::server_api::ai::AmbientAgentTask,
    workspace::WorkspaceAction,
};

#[derive(Debug, Clone, Default)]
pub struct ConversationDetailsData;

impl ConversationDetailsData {
    pub fn from_conversation(_conversation: &AIConversation, _app: &AppContext) -> Self {
        Self
    }

    pub fn from_task(
        _task: &AmbientAgentTask,
        _open_action: Option<WorkspaceAction>,
        _copy_link_url: Option<String>,
        _app: &AppContext,
    ) -> Self {
        Self
    }

    pub fn from_task_id(_task_id: AmbientAgentTaskId) -> Self {
        Self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_conversation_metadata(
        _ai_conversation_id: AIConversationId,
        _title: String,
        _creator_name: Option<String>,
        _created_at: DateTime<Local>,
        _directory: Option<String>,
        _credits_used: Option<f32>,
        _conversation_id: Option<String>,
        _artifacts: Vec<Artifact>,
        _open_action: Option<WorkspaceAction>,
        _status: Option<ConversationStatus>,
        _initial_query: Option<String>,
        _copy_link_url: Option<String>,
        _harness: Option<Harness>,
    ) -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub enum ConversationDetailsPanelEvent {
    Close,
    OpenPlanNotebook { notebook_uid: NotebookId },
}

#[derive(Debug, Clone)]
pub enum ConversationDetailsPanelAction {
    Close,
}

pub fn init(_app: &mut AppContext) {}

pub struct ConversationDetailsPanel;

impl ConversationDetailsPanel {
    pub fn new(_show_open_button: bool, _initial_width: f32, _ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn set_conversation_details(
        &mut self,
        _data: ConversationDetailsData,
        ctx: &mut ViewContext<Self>,
    ) {
        ctx.notify();
    }
}

impl Entity for ConversationDetailsPanel {
    type Event = ConversationDetailsPanelEvent;
}

impl View for ConversationDetailsPanel {
    fn ui_name() -> &'static str {
        "ConversationDetailsPanel"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for ConversationDetailsPanel {
    type Action = ConversationDetailsPanelAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            ConversationDetailsPanelAction::Close => {
                ctx.emit(ConversationDetailsPanelEvent::Close);
            }
        }
    }
}
