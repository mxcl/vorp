use std::rc::Rc;

use ai::agent::action::{RunAgentsAgentRunConfig, RunAgentsRequest};
use ai::agent::action_result::{RunAgentsAgentOutcomeKind, RunAgentsResult};
use ai::agent::orchestration_config::{
    matches_active_config, OrchestrationConfig, OrchestrationConfigStatus,
};
use ai::skills::SkillReference;
use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, ModelHandle, TypedActionView, View, ViewContext};

use crate::ai::agent::AIAgentActionId;
use crate::ai::blocklist::action_model::{BlocklistAIActionModel, RunAgentsExecutor};
use crate::ai::blocklist::block::model::AIBlockModel;
use crate::ai::blocklist::block::AIBlock;

pub fn init(_app: &mut AppContext) {}

#[derive(Clone, PartialEq, Eq)]
pub struct RunAgentsEditState {
    pub is_editor_open: bool,
    pub orch: crate::ai::blocklist::inline_action::orchestration_controls::OrchestrationEditState,
    pub agent_run_configs: Vec<RunAgentsAgentRunConfig>,
    pub base_prompt: String,
    pub summary: String,
    pub skills: Vec<SkillReference>,
}

impl std::fmt::Debug for RunAgentsEditState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RunState")
    }
}

impl RunAgentsEditState {
    pub fn from_request(req: &RunAgentsRequest) -> Self {
        Self {
            is_editor_open: false,
            orch: crate::ai::blocklist::inline_action::orchestration_controls::OrchestrationEditState::from_run_agents_fields(
                &req.model_id,
                &req.harness_type,
                &req.execution_mode,
            ),
            agent_run_configs: req.agent_run_configs.clone(),
            base_prompt: req.base_prompt.clone(),
            summary: req.summary.clone(),
            skills: req.skills.clone(),
        }
    }

    pub fn to_request(&self) -> RunAgentsRequest {
        RunAgentsRequest {
            summary: self.summary.clone(),
            base_prompt: self.base_prompt.clone(),
            skills: self.skills.clone(),
            model_id: self.orch.model_id.clone(),
            harness_type: self.orch.harness_type.clone(),
            execution_mode: self.orch.execution_mode.clone(),
            agent_run_configs: self.agent_run_configs.clone(),
        }
    }
}

#[derive(Clone)]
pub enum RunAgentsCardViewAction {
    Accept,
    Reject,
    ToggleEdit,
    DiscardEdits,
    ExecutionModeToggled { is_remote: bool },
    ModelChanged { model_id: String },
    HarnessChanged { harness_type: String },
    EnvironmentChanged { environment_id: String },
    WorkerHostChanged { worker_host: String },
}

impl std::fmt::Debug for RunAgentsCardViewAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RunAction")
    }
}

#[derive(Clone)]
pub enum RunAgentsCardViewEvent {
    RejectRequested,
}

impl std::fmt::Debug for RunAgentsCardViewEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RunEvent")
    }
}

pub struct RunAgentsCardView {
    state: RunAgentsEditState,
}

pub(crate) fn should_auto_launch(
    auto_launched: bool,
    is_denied: bool,
    is_spawning: bool,
    state: &RunAgentsEditState,
    active_config: &Option<(OrchestrationConfig, OrchestrationConfigStatus)>,
) -> bool {
    if auto_launched
        || is_denied
        || is_spawning
        || state.is_editor_open
        || state.agent_run_configs.is_empty()
    {
        return false;
    }
    match active_config {
        Some((config, status)) => {
            let request = state.to_request();
            status.is_approved() && matches_active_config(&request, config)
        }
        None => false,
    }
}

pub(crate) fn compute_is_denied(
    has_denied_result: bool,
    active_config: &Option<(OrchestrationConfig, OrchestrationConfigStatus)>,
) -> bool {
    has_denied_result
        || matches!(
            active_config,
            Some((_, status)) if status.is_disapproved()
        )
}

impl RunAgentsCardView {
    pub fn new(
        _action_id: AIAgentActionId,
        request: &RunAgentsRequest,
        _active_config: Option<(OrchestrationConfig, OrchestrationConfigStatus)>,
        _action_model: ModelHandle<BlocklistAIActionModel>,
        _run_agents_executor: ModelHandle<RunAgentsExecutor>,
        _block_model: Rc<dyn AIBlockModel<View = AIBlock>>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            state: RunAgentsEditState::from_request(request),
        }
    }

    pub fn is_spawning(&self) -> bool {
        false
    }

    pub fn update_request(&mut self, request: &RunAgentsRequest, ctx: &mut ViewContext<Self>) {
        self.state = RunAgentsEditState::from_request(request);
        ctx.notify();
    }

    pub fn try_auto_launch_on_stream_complete(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn accept(&mut self, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for RunAgentsCardView {
    type Event = RunAgentsCardViewEvent;
}

impl View for RunAgentsCardView {
    fn ui_name() -> &'static str {
        "RunView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for RunAgentsCardView {
    type Action = RunAgentsCardViewAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        if matches!(action, RunAgentsCardViewAction::Reject) {
            ctx.emit(RunAgentsCardViewEvent::RejectRequested);
        }
    }
}

pub(crate) fn format_terminal_state(result: &RunAgentsResult) -> (String, StatusKind) {
    match result {
        RunAgentsResult::Launched { agents, .. } => {
            let total = agents.len();
            let launched = agents
                .iter()
                .filter(|a| matches!(a.kind, RunAgentsAgentOutcomeKind::Launched { .. }))
                .count();
            let kind = if launched == total {
                StatusKind::Success
            } else {
                StatusKind::Mixed
            };
            (String::new(), kind)
        }
        RunAgentsResult::Denied { .. } | RunAgentsResult::Cancelled => {
            (String::new(), StatusKind::Cancelled)
        }
        RunAgentsResult::Failure { .. } => (String::new(), StatusKind::Failure),
    }
}

#[derive(Clone, Copy)]
pub(crate) enum StatusKind {
    Spawning,
    Success,
    Mixed,
    Failure,
    Cancelled,
}
