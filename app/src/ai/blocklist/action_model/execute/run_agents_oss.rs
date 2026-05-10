use ai::agent::action::{RunAgentsAgentRunConfig, RunAgentsExecutionMode, RunAgentsRequest};
use ai::agent::action_result::RunAgentsResult;
use ai::skills::SkillReference;
use futures::{future::BoxFuture, FutureExt};
use warpui::{Entity, ModelContext, ModelHandle};

use super::start_agent::StartAgentExecutor;
use super::{ActionExecution, AnyActionExecution, ExecuteActionInput, PreprocessActionInput};
use crate::ai::agent::conversation::AIConversationId;
use crate::ai::agent::{AIAgentActionId, AIAgentActionResultType, AIAgentActionType};

#[derive(Debug, Clone, Copy)]
pub struct RunAgentsSpawningSnapshot {
    pub agent_count: usize,
}

pub struct RunAgentsExecutor;

pub enum RunAgentsExecutorEvent {
    SpawningStarted {
        action_id: AIAgentActionId,
        snapshot: RunAgentsSpawningSnapshot,
    },
    SpawningFinished {
        action_id: AIAgentActionId,
    },
}

impl Entity for RunAgentsExecutor {
    type Event = RunAgentsExecutorEvent;
}

impl RunAgentsExecutor {
    pub fn new(_start_agent_executor: ModelHandle<StartAgentExecutor>) -> Self {
        Self
    }

    pub fn is_pending(&self, _action_id: &AIAgentActionId) -> bool {
        false
    }

    pub fn dispatch_run_agents(
        &mut self,
        _action_id: AIAgentActionId,
        _request: RunAgentsRequest,
        _parent_conversation_id: AIConversationId,
        _ctx: &mut ModelContext<Self>,
    ) -> async_channel::Receiver<RunAgentsResult> {
        let (sender, receiver) = async_channel::bounded(1);
        let _ = sender.try_send(RunAgentsResult::Cancelled);
        receiver
    }

    pub(super) fn execute(
        &mut self,
        input: ExecuteActionInput,
        _ctx: &mut ModelContext<Self>,
    ) -> impl Into<AnyActionExecution> {
        let AIAgentActionType::RunAgents(_) = input.action.action else {
            return ActionExecution::<()>::InvalidAction;
        };
        ActionExecution::<()>::Sync(AIAgentActionResultType::RunAgents(
            RunAgentsResult::Cancelled,
        ))
    }

    pub(super) fn should_autoexecute(
        &self,
        _input: ExecuteActionInput,
        _ctx: &mut ModelContext<Self>,
    ) -> bool {
        false
    }

    pub(super) fn preprocess_action(
        &mut self,
        _action: PreprocessActionInput,
        _ctx: &mut ModelContext<Self>,
    ) -> BoxFuture<'static, ()> {
        futures::future::ready(()).boxed()
    }
}

#[cfg(test)]
pub fn compose_run_agents_child_prompt(base_prompt: &str, per_agent_prompt: &str) -> String {
    let base_trimmed = base_prompt.trim();
    let per_agent_trimmed = per_agent_prompt.trim();
    match (base_trimmed.is_empty(), per_agent_trimmed.is_empty()) {
        (false, false) => format!("{base_prompt}\n\n{per_agent_prompt}"),
        (false, true) => base_prompt.to_string(),
        (true, false) => per_agent_prompt.to_string(),
        (true, true) => String::new(),
    }
}

#[cfg(test)]
pub fn run_agents_to_start_agent_mode(
    _run_execution_mode: &RunAgentsExecutionMode,
    _run_harness_type: &str,
    _run_model_id: &str,
    _run_skills: &[SkillReference],
    _cfg: &RunAgentsAgentRunConfig,
) -> Result<crate::ai::agent::StartAgentExecutionMode, String> {
    Err("RunAgents is disabled in OSS builds".to_string())
}
