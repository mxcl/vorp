use futures::{future::BoxFuture, FutureExt};
use warpui::{Entity, ModelContext};

use super::{ActionExecution, AnyActionExecution, ExecuteActionInput, PreprocessActionInput};
use crate::ai::agent::conversation::AIConversationId;
use crate::ai::agent::{
    AIAgentAction, AIAgentActionResultType, AIAgentActionType, LifecycleEventType,
    StartAgentExecutionMode, StartAgentResult,
};

#[derive(Debug, Clone)]
pub enum StartAgentOutcome {
    Started { agent_id: String },
    Error(String),
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Default)]
pub struct StartAgentRequestId(u64);

impl StartAgentRequestId {
    #[cfg(test)]
    pub const fn from_raw_for_test(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Clone)]
pub struct StartAgentRequest {
    pub id: StartAgentRequestId,
    pub name: String,
    pub prompt: String,
    pub execution_mode: StartAgentExecutionMode,
    pub lifecycle_subscription: Option<Vec<LifecycleEventType>>,
    pub parent_conversation_id: AIConversationId,
    pub parent_run_id: Option<String>,
}

pub struct StartAgentExecutor;

impl StartAgentExecutor {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }

    pub(super) fn should_autoexecute(
        &self,
        _input: ExecuteActionInput,
        _ctx: &mut ModelContext<Self>,
    ) -> bool {
        false
    }

    pub(super) fn execute(
        &mut self,
        input: ExecuteActionInput,
        _ctx: &mut ModelContext<Self>,
    ) -> impl Into<AnyActionExecution> {
        let AIAgentAction {
            action: AIAgentActionType::StartAgent { version, .. },
            ..
        } = input.action
        else {
            return ActionExecution::InvalidAction;
        };

        ActionExecution::<()>::Sync(AIAgentActionResultType::StartAgent(
            StartAgentResult::Error {
                error: "StartAgent is disabled in OSS builds".to_string(),
                version: *version,
            },
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn dispatch(
        &mut self,
        _name: String,
        _prompt: String,
        _execution_mode: StartAgentExecutionMode,
        _lifecycle_subscription: Option<Vec<LifecycleEventType>>,
        _parent_conversation_id: AIConversationId,
        _parent_run_id: Option<String>,
        _ctx: &mut ModelContext<Self>,
    ) -> async_channel::Receiver<StartAgentOutcome> {
        let (sender, receiver) = async_channel::bounded(1);
        let _ = sender.try_send(StartAgentOutcome::Error(
            "StartAgent is disabled in OSS builds".to_string(),
        ));
        receiver
    }

    pub(super) fn preprocess_action(
        &mut self,
        _action: PreprocessActionInput,
        _ctx: &mut ModelContext<Self>,
    ) -> BoxFuture<'static, ()> {
        futures::future::ready(()).boxed()
    }
}

impl Entity for StartAgentExecutor {
    type Event = StartAgentExecutorEvent;
}

pub enum StartAgentExecutorEvent {
    CreateAgent(StartAgentRequest),
}
