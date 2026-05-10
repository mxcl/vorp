use serde::{Deserialize, Serialize};

use crate::{ai::block_context::BlockContext, terminal::input::IntelligentAutosuggestionResult};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenerateAIInputSuggestionsRequest {
    pub context_messages: Vec<String>,
    pub history_context: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_context: Option<String>,
    pub rejected_suggestions: Vec<String>,
    pub prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_context: Option<Box<BlockContext>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_result: Option<IntelligentAutosuggestionResult>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentModeSuggestionV2 {
    pub query: String,
    pub context_block_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenerateAIInputSuggestionsResponseV2 {
    pub commands: Vec<String>,
    pub ai_queries: Vec<AgentModeSuggestionV2>,
    pub most_likely_action: String,
}

#[derive(Clone)]
pub struct HistoryContext {
    pub previous_commands: Vec<crate::persistence::model::Command>,
    pub next_command: crate::persistence::model::Command,
}
