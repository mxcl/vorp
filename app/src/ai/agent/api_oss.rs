use std::{collections::HashMap, pin::Pin, sync::Arc};

use chrono::{DateTime, Local};
use futures_lite::Stream;
use serde::Serialize;
use warp_core::channel::ChannelState;

use crate::{
    ai::{
        agent::{
            conversation::{AIConversation, AIConversationId, ServerAIConversationMetadata},
            task::TaskId,
            AIAgentExchange, AIAgentInput, AIAgentOutputMessage, AIAgentTodo,
            SuggestedAgentModeWorkflow, SuggestedRule,
        },
        ambient_agents::AmbientAgentTaskId,
        blocklist::{RequestInput, SessionContext},
        document::ai_document_model::{AIDocumentId, AIDocumentVersion},
        llms::LLMId,
    },
    server::server_api::{AIApiError, ServerApi},
};

pub use ai::agent::convert::ConvertToAPITypeError;

/// Unique, server-generated conversation-scoped token to be roundtripped to the API when sending
/// requests that follow-up within a given conversation.
#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServerConversationToken(String);

impl ServerConversationToken {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn debug_link(&self) -> String {
        format!(
            "{}/debug/maa/{}",
            ChannelState::server_root_url(),
            self.as_str()
        )
    }

    pub fn conversation_link(&self) -> String {
        format!(
            "{}/conversation/{}",
            ChannelState::server_root_url(),
            self.as_str()
        )
    }
}

impl From<ServerConversationToken> for String {
    fn from(value: ServerConversationToken) -> Self {
        value.0
    }
}

impl From<session_sharing_protocol::common::ServerConversationToken> for ServerConversationToken {
    fn from(token: session_sharing_protocol::common::ServerConversationToken) -> Self {
        Self(token.to_string())
    }
}

impl TryFrom<ServerConversationToken>
    for session_sharing_protocol::common::ServerConversationToken
{
    type Error = uuid::Error;

    fn try_from(token: ServerConversationToken) -> Result<Self, Self::Error> {
        token.as_str().parse()
    }
}

#[derive(Debug, Clone)]
pub struct ConversationData {
    pub id: AIConversationId,
    pub tasks: Vec<warp_multi_agent_api::Task>,
    pub server_conversation_token: Option<ServerConversationToken>,
    pub forked_from_conversation_token: Option<ServerConversationToken>,
    pub ambient_agent_task_id: Option<AmbientAgentTaskId>,
    pub existing_suggestions: Option<super::Suggestions>,
}

#[derive(Debug, Clone)]
pub struct RequestParams {
    pub model: LLMId,
    pub parent_agent_id: Option<String>,
    pub agent_name: Option<String>,
}

impl RequestParams {
    pub fn new(
        _terminal_view_id: Option<warpui::EntityId>,
        _session_context: SessionContext,
        _request_input: &RequestInput,
        _conversation: ConversationData,
        _metadata: Option<super::RequestMetadata>,
        _app: &warpui::AppContext,
    ) -> Self {
        Self {
            model: LLMId::from("oss-disabled"),
            parent_agent_id: None,
            agent_name: None,
        }
    }
}

pub type Event = Result<warp_multi_agent_api::ResponseEvent, Arc<AIApiError>>;

#[cfg(not(target_family = "wasm"))]
pub type ResponseStream = Pin<Box<dyn Stream<Item = Event> + Send + 'static>>;

#[cfg(target_family = "wasm")]
pub type ResponseStream = Pin<Box<dyn Stream<Item = Event>>>;

pub async fn generate_multi_agent_output(
    _server_api: Arc<ServerApi>,
    _params: RequestParams,
    _cancellation_rx: futures::channel::oneshot::Receiver<()>,
) -> Result<ResponseStream, ConvertToAPITypeError> {
    Err(ConvertToAPITypeError::Unimplemented(
        "AI API is disabled in OSS builds".to_string(),
    ))
}

#[derive(Debug, thiserror::Error)]
pub enum MessageToAIAgentOutputMessageError {
    #[error("AI API conversion is disabled in OSS builds")]
    Disabled,
}

#[allow(clippy::large_enum_variant)]
pub enum MaybeAIAgentOutputMessage {
    Message(AIAgentOutputMessage),
    NoClientRepresentation,
}

pub struct ConversionParams<'a> {
    pub task_id: &'a TaskId,
    pub current_todo_list: Option<&'a super::todos::AIAgentTodoList>,
    pub active_code_review: Option<&'a super::comment::CodeReview>,
}

pub trait ConvertAPIMessageToClientOutputMessage {
    fn to_client_output_message(
        self,
        params: ConversionParams<'_>,
    ) -> Result<MaybeAIAgentOutputMessage, MessageToAIAgentOutputMessageError>;
}

impl ConvertAPIMessageToClientOutputMessage for warp_multi_agent_api::Message {
    fn to_client_output_message(
        self,
        params: ConversionParams<'_>,
    ) -> Result<MaybeAIAgentOutputMessage, MessageToAIAgentOutputMessageError> {
        let _ = (
            params.task_id,
            params.current_todo_list,
            params.active_code_review,
        );
        Ok(MaybeAIAgentOutputMessage::NoClientRepresentation)
    }
}

pub fn user_inputs_from_messages(_messages: &[warp_multi_agent_api::Message]) -> Vec<AIAgentInput> {
    Vec::new()
}

impl From<warp_multi_agent_api::TodoItem> for AIAgentTodo {
    fn from(value: warp_multi_agent_api::TodoItem) -> Self {
        AIAgentTodo {
            id: value.id.into(),
            title: value.title,
            description: value.description,
        }
    }
}

impl From<warp_multi_agent_api::Suggestions> for super::Suggestions {
    fn from(api_suggestions: warp_multi_agent_api::Suggestions) -> Self {
        Self {
            rules: api_suggestions
                .rules
                .into_iter()
                .map(|rule| SuggestedRule {
                    name: rule.name,
                    content: rule.content,
                    logging_id: rule.logging_id.into(),
                })
                .collect(),
            agent_mode_workflows: api_suggestions
                .workflows
                .into_iter()
                .map(|workflow| SuggestedAgentModeWorkflow {
                    name: workflow.name,
                    prompt: workflow.prompt,
                    logging_id: workflow.logging_id.into(),
                })
                .collect(),
        }
    }
}

pub(crate) mod convert_conversation {
    use super::*;

    pub enum RestorationMode {
        Continue,
        #[allow(dead_code)]
        Fork,
    }

    pub fn convert_conversation_data_to_ai_conversation(
        _conversation_id: AIConversationId,
        _conversation_data: &warp_multi_agent_api::ConversationData,
        _metadata: ServerAIConversationMetadata,
        _restoration_mode: RestorationMode,
    ) -> Option<AIConversation> {
        None
    }

    pub trait ConvertToExchanges {
        fn into_exchanges(self) -> Vec<AIAgentExchange>;
    }

    impl ConvertToExchanges for &warp_multi_agent_api::Task {
        fn into_exchanges(self) -> Vec<AIAgentExchange> {
            Vec::new()
        }
    }

    pub(crate) fn convert_tool_call_result_to_input(
        _task_id: &TaskId,
        _tool_call_result: &warp_multi_agent_api::message::ToolCallResult,
        _tool_call_map: &HashMap<String, &warp_multi_agent_api::message::ToolCall>,
        _document_versions: &mut HashMap<AIDocumentId, AIDocumentVersion>,
    ) -> Option<AIAgentInput> {
        None
    }

    pub(crate) fn compute_time_to_first_token_ms_from_messages<'a, I>(
        _start_time: DateTime<Local>,
        _messages: I,
    ) -> Option<i64>
    where
        I: Iterator<Item = &'a warp_multi_agent_api::Message>,
    {
        None
    }
}
