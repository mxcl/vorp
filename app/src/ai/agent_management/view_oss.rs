use crate::{
    ai::{
        agent::conversation::AIConversationId, agent_conversations_model::AgentManagementFilters,
        ambient_agents::AmbientAgentTaskId,
    },
    app_state::PersistedAgentManagementFilters,
    notebooks::NotebookId,
    workflows::WorkflowType,
};
use warpui::{elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext};

pub fn init(_app: &mut AppContext) {}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ManagementCardItemId {
    Task(AmbientAgentTaskId),
    Conversation(AIConversationId),
}

impl std::fmt::Debug for ManagementCardItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ManagementCardItemId")
    }
}

pub struct AgentManagementView {
    filters: AgentManagementFilters,
}

impl AgentManagementView {
    pub fn new(
        persisted_filters: Option<PersistedAgentManagementFilters>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            filters: persisted_filters
                .map(|persisted| persisted.filters)
                .unwrap_or_default(),
        }
    }

    pub fn get_filters(&self) -> PersistedAgentManagementFilters {
        PersistedAgentManagementFilters {
            filters: self.filters.clone(),
        }
    }

    pub(crate) fn show_setup_guide_from_link(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.notify();
    }

    pub(crate) fn apply_environment_filter_from_link(
        &mut self,
        _environment_id: String,
        ctx: &mut ViewContext<Self>,
    ) {
        ctx.notify();
    }
}

impl Entity for AgentManagementView {
    type Event = AgentManagementViewEvent;
}

impl View for AgentManagementView {
    fn ui_name() -> &'static str {
        "ManagementView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

#[derive(Clone, PartialEq)]
pub enum AgentManagementViewAction {
    Noop,
}

impl std::fmt::Debug for AgentManagementViewAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ManagementViewAction")
    }
}

pub enum AgentManagementViewEvent {
    OpenNewTabAndRunWorkflow(Box<WorkflowType>),
    OpenPlanNotebook { notebook_uid: NotebookId },
}

impl TypedActionView for AgentManagementView {
    type Action = AgentManagementViewAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}
