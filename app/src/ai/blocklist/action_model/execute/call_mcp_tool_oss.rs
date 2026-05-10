use super::{ActionExecution, AnyActionExecution, ExecuteActionInput, PreprocessActionInput};
use crate::{
    ai::agent::{AIAgentActionResultType, CallMCPToolResult},
    terminal::model::session::active_session::ActiveSession,
};
use futures::{future::BoxFuture, FutureExt};
use warpui::{Entity, EntityId, ModelContext, ModelHandle};

pub struct CallMCPToolExecutor {
    _active_session: ModelHandle<ActiveSession>,
    _terminal_view_id: EntityId,
}

impl CallMCPToolExecutor {
    pub fn new(active_session: ModelHandle<ActiveSession>, terminal_view_id: EntityId) -> Self {
        Self {
            _active_session: active_session,
            _terminal_view_id: terminal_view_id,
        }
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
        _input: ExecuteActionInput,
        _ctx: &mut ModelContext<Self>,
    ) -> impl Into<AnyActionExecution> {
        ActionExecution::<()>::Sync(AIAgentActionResultType::CallMCPTool(
            CallMCPToolResult::Error("MCP runtime is not available in this build".to_owned()),
        ))
    }

    pub(super) fn preprocess_action(
        &mut self,
        _action: PreprocessActionInput,
        _ctx: &mut ModelContext<Self>,
    ) -> BoxFuture<'static, ()> {
        futures::future::ready(()).boxed()
    }
}

impl Entity for CallMCPToolExecutor {
    type Event = ();
}

pub(crate) fn coerce_integer_args(
    _args: &mut serde_json::Map<String, serde_json::Value>,
    _input_schema: &serde_json::Map<String, serde_json::Value>,
) {
}
