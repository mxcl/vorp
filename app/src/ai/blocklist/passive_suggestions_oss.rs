use std::sync::Arc;

use parking_lot::FairMutex;
use warpui::{Entity, EntityId, ModelContext, ModelHandle};

use crate::{
    ai::{
        agent::{
            conversation::AIConversationId, AIAgentExchangeId, PassiveCodeDiffEntry,
            PassiveSuggestionTrigger,
        },
        blocklist::{
            controller::BlocklistAIController, inline_action::code_diff_view::FileDiff,
            RequestFileEditsFormatKind, ResponseStreamId,
        },
    },
    server::telemetry::PromptSuggestionFallbackReason,
    terminal::{
        model::{
            block::BlockId, session::active_session::ActiveSession, terminal_model::TerminalModel,
        },
        model_events::ModelEventDispatcher,
        view::{ambient_agent::AmbientAgentViewModel, AgentModePromptSuggestion},
    },
};

#[derive(Clone)]
pub struct PassiveSuggestionsModels {
    pub legacy: ModelHandle<LegacyPassiveSuggestionsModel>,
    pub maa: ModelHandle<MaaPassiveSuggestionsModel>,
}

pub enum LegacyPassiveSuggestionsEvent {
    PromptSuggestionsGenerated {
        prompt_suggestion: AgentModePromptSuggestion,
        block_id: BlockId,
        command: String,
        request_duration_ms: u64,
    },
    PassiveCodeDiffRequestStarted {
        prompt_suggestion_id: String,
        code_exchange_id: Option<AIAgentExchangeId>,
        block_id: BlockId,
    },
    PassiveCodeDiffFailed {
        reason: PromptSuggestionFallbackReason,
    },
}

pub struct LegacyPassiveSuggestionsModel;

impl LegacyPassiveSuggestionsModel {
    pub fn new(
        _active_session: ModelHandle<ActiveSession>,
        _terminal_model: Arc<FairMutex<TerminalModel>>,
        _ai_controller: ModelHandle<BlocklistAIController>,
        _model_event_dispatcher: &ModelHandle<ModelEventDispatcher>,
        _terminal_view_id: EntityId,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self
    }

    pub fn is_passive_code_diff_being_generated(&self) -> bool {
        false
    }

    pub fn abort_pending_requests(
        &mut self,
        _ctx: &mut ModelContext<Self>,
    ) -> Vec<ResponseStreamId> {
        Vec::new()
    }
}

impl Entity for LegacyPassiveSuggestionsModel {
    type Event = LegacyPassiveSuggestionsEvent;
}

pub enum MaaPassiveSuggestionsEvent {
    NewPromptSuggestion {
        prompt: String,
        label: Option<String>,
        request_duration_ms: u64,
        trigger: Option<PassiveSuggestionTrigger>,
        conversation_id: Option<AIConversationId>,
        server_request_token: Option<String>,
    },
    NewCodeDiffSuggestion {
        diffs: Vec<FileDiff>,
        edit_format_kind: RequestFileEditsFormatKind,
        title: Option<String>,
        original_edits: Vec<PassiveCodeDiffEntry>,
        conversation_id: Option<AIConversationId>,
        request_duration_ms: u64,
        trigger: PassiveSuggestionTrigger,
        server_request_token: Option<String>,
    },
}

pub struct MaaPassiveSuggestionsModel;

impl MaaPassiveSuggestionsModel {
    pub fn new(
        _active_session: ModelHandle<ActiveSession>,
        _terminal_model: Arc<FairMutex<TerminalModel>>,
        _ai_controller: ModelHandle<BlocklistAIController>,
        _model_event_dispatcher: &ModelHandle<ModelEventDispatcher>,
        _ambient_agent_view_model: Option<ModelHandle<AmbientAgentViewModel>>,
        _terminal_view_id: EntityId,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self
    }

    pub fn abort_pending_requests(&mut self, _ctx: &mut ModelContext<Self>) {}
}

impl Entity for MaaPassiveSuggestionsModel {
    type Event = MaaPassiveSuggestionsEvent;
}
