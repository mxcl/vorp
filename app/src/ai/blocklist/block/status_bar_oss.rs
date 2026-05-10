use std::sync::Arc;

use parking_lot::FairMutex;
use warpui::elements::{Element, Empty};
use warpui::{
    AppContext, Entity, EntityId, ModelHandle, TypedActionView, View, ViewContext, ViewHandle,
};

use crate::ai::blocklist::agent_view::{
    shortcuts::AgentShortcutViewModel, AgentViewController, EphemeralMessageModel,
};
use crate::ai::blocklist::summarization_cancel_dialog::{
    SummarizationCancelDialog, SummarizationCancelDialogEvent,
};
use crate::ai::blocklist::{
    BlocklistAIActionModel, BlocklistAIContextModel, BlocklistAIController, BlocklistAIInputModel,
};
use crate::terminal::input::buffer_model::InputBufferModel;
use crate::terminal::input::slash_command_model::SlashCommandModel;
use crate::terminal::input::suggestions_mode_model::InputSuggestionsModeModel;
use crate::terminal::model::block::BlockId;
use crate::terminal::model_events::ModelEventDispatcher;
use crate::terminal::view::ambient_agent::AmbientAgentViewModel;
use crate::terminal::TerminalModel;

use super::cli_controller::CLISubagentController;

pub fn init(app: &mut AppContext) {
    crate::ai::blocklist::summarization_cancel_dialog::init(app);
}

pub struct BlocklistAIStatusBar {
    summarization_cancel_dialog: ViewHandle<SummarizationCancelDialog>,
}

impl BlocklistAIStatusBar {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        _controller: ModelHandle<BlocklistAIController>,
        _agent_view_controller: ModelHandle<AgentViewController>,
        _cli_subagent_controller: ModelHandle<CLISubagentController>,
        _action_model: ModelHandle<BlocklistAIActionModel>,
        _context_model: ModelHandle<BlocklistAIContextModel>,
        _input_model: ModelHandle<BlocklistAIInputModel>,
        _input_buffer_model: ModelHandle<InputBufferModel>,
        _model_event_dispatcher: &ModelHandle<ModelEventDispatcher>,
        _terminal_model: Arc<FairMutex<TerminalModel>>,
        _shortcut_view_model: ModelHandle<AgentShortcutViewModel>,
        _ambient_agent_view_model: Option<ModelHandle<AmbientAgentViewModel>>,
        _input_suggestions_model: ModelHandle<InputSuggestionsModeModel>,
        _slash_command_model: ModelHandle<SlashCommandModel>,
        _ephemeral_message_model: ModelHandle<EphemeralMessageModel>,
        _terminal_view_id: EntityId,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        let summarization_cancel_dialog =
            ctx.add_typed_action_view(|_| SummarizationCancelDialog::default());
        ctx.subscribe_to_view(
            &summarization_cancel_dialog,
            |_, _, event, ctx| match event {
                SummarizationCancelDialogEvent::ConfirmCancel
                | SummarizationCancelDialogEvent::Continue => {
                    ctx.emit(
                        BlocklistAIStatusBarEvent::SummarizationCancelDialogToggled {
                            is_open: false,
                        },
                    );
                }
            },
        );

        Self {
            summarization_cancel_dialog,
        }
    }

    pub fn should_show_summarization_cancel_dialog(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn summarization_cancel_dialog_handle(&self) -> &ViewHandle<SummarizationCancelDialog> {
        &self.summarization_cancel_dialog
    }

    pub fn handle_ctrl_c(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(BlocklistAIStatusBarEvent::Stop);
    }

    pub fn notify_and_notify_children(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.notify();
    }
}

impl View for BlocklistAIStatusBar {
    fn ui_name() -> &'static str {
        "BlocklistAIStatusBar"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

#[derive(Debug, Clone)]
pub enum BlocklistAIStatusBarEvent {
    SummarizationCancelDialogToggled { is_open: bool },
    Stop,
}

impl Entity for BlocklistAIStatusBar {
    type Event = BlocklistAIStatusBarEvent;
}

#[derive(Debug, Clone)]
pub enum BlocklistAIStatusBarAction {
    ToggleHideResponses,
    SwitchCommandControlToUser,
    Stop,
    ForceRefreshAgentView { block_id: BlockId },
}

impl TypedActionView for BlocklistAIStatusBar {
    type Action = BlocklistAIStatusBarAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        if matches!(action, BlocklistAIStatusBarAction::Stop) {
            ctx.emit(BlocklistAIStatusBarEvent::Stop);
        }
    }
}
