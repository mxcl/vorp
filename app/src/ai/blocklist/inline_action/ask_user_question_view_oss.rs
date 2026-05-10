use ai::agent::action::AskUserQuestionItem;
use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, ModelHandle, TypedActionView, View, ViewContext};

use crate::ai::agent::{conversation::AIConversationId, AIAgentActionId};
use crate::ai::blocklist::action_model::BlocklistAIActionModel;

pub fn init(_app: &mut AppContext) {}

#[derive(Clone)]
pub enum QuestionAction {
    OptionToggled { option_index: usize },
    SelectionConfirmed,
    SkipAll,
    FreeTextSubmitted { text: String },
    OtherSelected,
    NavigateNext,
    NavigatePrev,
    ToggleExpanded,
    EnterPressed,
}

pub type AskUserQuestionViewAction = QuestionAction;

impl std::fmt::Debug for QuestionAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Action")
    }
}

#[derive(Clone)]
pub enum QuestionEvent {
    Updated,
}

pub type AskUserQuestionViewEvent = QuestionEvent;

impl std::fmt::Debug for QuestionEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Event")
    }
}

pub(crate) type AskUserQuestionView = QuestionView;

pub(crate) struct QuestionView {
    action_id: AIAgentActionId,
    questions: Vec<AskUserQuestionItem>,
}

impl QuestionView {
    pub fn new(
        _action_model: ModelHandle<BlocklistAIActionModel>,
        _conversation_id: AIConversationId,
        action_id: AIAgentActionId,
        questions: Vec<AskUserQuestionItem>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            action_id,
            questions,
        }
    }

    pub fn action_id(&self) -> &AIAgentActionId {
        &self.action_id
    }

    pub fn is_editing(&self) -> bool {
        false
    }

    pub fn should_render_inline(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn matches_action(
        &self,
        action_id: &AIAgentActionId,
        questions: &[AskUserQuestionItem],
    ) -> bool {
        self.action_id() == action_id && self.questions == questions
    }
}

impl Entity for QuestionView {
    type Event = QuestionEvent;
}

impl View for QuestionView {
    fn ui_name() -> &'static str {
        "InlineView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for QuestionView {
    type Action = QuestionAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}
