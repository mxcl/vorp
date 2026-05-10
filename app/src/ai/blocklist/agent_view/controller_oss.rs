use std::{fmt, sync::Arc, time::Duration};

use parking_lot::FairMutex;
use warpui::{Entity, EntityId, ModelContext, ModelHandle};

use crate::{
    ai::agent::conversation::AIConversationId,
    terminal::{input::slash_commands::SlashCommandTrigger, TerminalModel},
    BlocklistAIHistoryModel,
};

use super::EphemeralMessageModel;

pub const ENTER_OR_EXIT_CONFIRMATION_WINDOW: Duration = Duration::from_secs(1);

#[derive(Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum EnterAgentViewError {
    #[error("Unavailable in this build.")]
    Unavailable,
    #[error("Unavailable in this build.")]
    AlreadyInAgentView,
    #[error("Unavailable in this build.")]
    LongRunningCommand,
}

impl fmt::Debug for EnterAgentViewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("EnterAgentViewError")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ExitAgentViewError {
    #[error("Unavailable in this build.")]
    LongRunningCommand,
    #[error("Unavailable in this build.")]
    ConversationViewer,
    #[error("Unavailable in this build.")]
    AmbientAgent,
}

impl fmt::Debug for ExitAgentViewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ExitAgentViewError")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AgentViewDisplayMode {
    FullScreen,
    Inline,
}

impl AgentViewDisplayMode {
    pub fn is_inline(self) -> bool {
        matches!(self, AgentViewDisplayMode::Inline)
    }

    pub fn is_fullscreen(self) -> bool {
        matches!(self, AgentViewDisplayMode::FullScreen)
    }
}

impl fmt::Debug for AgentViewDisplayMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AgentViewDisplayMode")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExitConfirmationTrigger {
    Escape,
    CtrlC,
}

impl fmt::Debug for ExitConfirmationTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ExitConfirmationTrigger")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AgentViewEntryOrigin {
    Input { was_prompt_autodetected: bool },
    PromptChip,
    ConversationSelector,
    AgentModeHomepage,
    AgentViewBlock,
    AIDocument,
    AutoFollowUp,
    RestoreExistingConversation,
    SharedSessionSelection,
    AgentRequestedNewConversation,
    AcceptedPromptSuggestion,
    AcceptedUnitTestSuggestion,
    AcceptedPassiveCodeDiff,
    InlineCodeReview,
    CloudAgent,
    ThirdPartyCloudAgent,
    Cli,
    ImageAdded,
    SlashCommand { trigger: SlashCommandTrigger },
    SlashInit,
    CreateEnvironment,
    Keybinding,
    CodeReviewContext,
    CodexModal,
    InlineHistoryMenu,
    InlineConversationMenu,
    OnboardingCallout,
    ConversationListView,
    DefaultSessionMode,
    LongRunningCommand,
    Onboarding,
    ChildAgent,
    OrchestrationPillBar,
    ProjectEntry,
    LinearDeepLink,
    ClearBuffer,
    ContinueConversationButton,
    ViewPassiveCodeDiffDetails,
    ResumeConversationButton,
}

impl AgentViewEntryOrigin {
    pub fn is_cloud_agent(&self) -> bool {
        false
    }

    pub fn should_autotrigger_request(&self) -> AutoTriggerBehavior {
        AutoTriggerBehavior::Never
    }
}

impl fmt::Debug for AgentViewEntryOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AgentViewEntryOrigin")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AutoTriggerBehavior {
    Always,
    InAgentView,
    Never,
}

impl fmt::Debug for AutoTriggerBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AutoTriggerBehavior")
    }
}

#[derive(Clone)]
pub enum AgentViewState {
    Active {
        conversation_id: AIConversationId,
        origin: AgentViewEntryOrigin,
        display_mode: AgentViewDisplayMode,
        original_conversation_length: usize,
    },
    Inactive,
}

impl AgentViewState {
    pub fn active_conversation_id(&self) -> Option<AIConversationId> {
        None
    }

    pub fn display_mode(&self) -> Option<AgentViewDisplayMode> {
        None
    }

    pub fn is_active(&self) -> bool {
        false
    }

    pub fn is_inline(&self) -> bool {
        false
    }

    pub fn is_fullscreen(&self) -> bool {
        false
    }

    pub fn fullscreen_conversation_id(&self) -> Option<AIConversationId> {
        None
    }

    pub fn is_new(&self) -> bool {
        false
    }

    pub fn was_conversation_modified_since_opening(
        &self,
        _history_model: &BlocklistAIHistoryModel,
    ) -> bool {
        false
    }

    pub fn zero_state_position_id(&self) -> Option<String> {
        None
    }
}

impl fmt::Debug for AgentViewState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AgentViewState")
    }
}

pub struct AgentViewController {
    terminal_view_id: EntityId,
    pane_group_id: Option<EntityId>,
    agent_view_state: AgentViewState,
}

impl AgentViewController {
    pub fn new(
        _terminal_model: Arc<FairMutex<TerminalModel>>,
        terminal_view_id: EntityId,
        _ephemeral_message_model: ModelHandle<EphemeralMessageModel>,
    ) -> Self {
        Self {
            terminal_view_id,
            pane_group_id: None,
            agent_view_state: AgentViewState::Inactive,
        }
    }

    pub fn pane_group_id(&self) -> Option<EntityId> {
        self.pane_group_id
    }

    pub fn terminal_view_id(&self) -> EntityId {
        self.terminal_view_id
    }

    pub fn set_pane_group_id(&mut self, pane_group_id: EntityId) {
        self.pane_group_id = Some(pane_group_id);
    }

    pub fn is_active(&self) -> bool {
        false
    }

    pub fn is_inline(&self) -> bool {
        false
    }

    pub fn is_fullscreen(&self) -> bool {
        false
    }

    pub fn agent_view_state(&self) -> &AgentViewState {
        &self.agent_view_state
    }

    pub fn can_exit_agent_view(&self) -> Result<(), ExitAgentViewError> {
        Ok(())
    }

    pub fn pending_exit_confirmation_conversation_id(&self) -> Option<AIConversationId> {
        None
    }

    pub fn clear_pending_exit_confirmation(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn should_start_new_conversation_for_keybinding(
        &mut self,
        _keybinding_name: &str,
        _ctx: &mut ModelContext<Self>,
    ) -> bool {
        false
    }

    pub fn try_enter_agent_view(
        &mut self,
        _conversation_id: Option<AIConversationId>,
        _origin: AgentViewEntryOrigin,
        _ctx: &mut ModelContext<Self>,
    ) -> Result<AIConversationId, EnterAgentViewError> {
        Err(EnterAgentViewError::Unavailable)
    }

    pub fn try_enter_inline_agent_view(
        &mut self,
        _conversation_id: Option<AIConversationId>,
        _origin: AgentViewEntryOrigin,
        _ctx: &mut ModelContext<Self>,
    ) -> Result<AIConversationId, EnterAgentViewError> {
        Err(EnterAgentViewError::Unavailable)
    }

    pub(crate) fn exit_agent_view_with_required_confirmation(
        &mut self,
        _trigger: ExitConfirmationTrigger,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub(crate) fn exit_agent_view_without_confirmation(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn exit_agent_view(&mut self, _ctx: &mut ModelContext<Self>) {}
}

#[derive(Clone)]
pub enum AgentViewControllerEvent {
    EnteredAgentView {
        conversation_id: AIConversationId,
        origin: AgentViewEntryOrigin,
        display_mode: AgentViewDisplayMode,
        is_new: bool,
    },
    ExitedAgentView {
        conversation_id: AIConversationId,
        origin: AgentViewEntryOrigin,
        display_mode: AgentViewDisplayMode,
        original_exchange_count: usize,
        final_exchange_count: usize,
        was_ambient_agent: bool,
        is_exit_before_new_entrance: bool,
    },
    ExitConfirmed {
        conversation_id: AIConversationId,
    },
}

impl fmt::Debug for AgentViewControllerEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AgentViewControllerEvent")
    }
}

impl Entity for AgentViewController {
    type Event = AgentViewControllerEvent;
}
