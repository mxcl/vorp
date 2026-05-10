use std::sync::Arc;

use parking_lot::FairMutex;
use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, EntityId, ModelHandle, TypedActionView, View, ViewContext};

use crate::ai::agent::{conversation::AIConversationId, task::TaskId};
use crate::ai::blocklist::block::cli_controller::CLISubagentController;
use crate::ai::blocklist::BlocklistAIActionModel;
use crate::code::editor_management::CodeSource;
use crate::terminal::model::block::BlockId;
use crate::terminal::{ShellLaunchData, TerminalModel};

pub fn init(_app: &mut AppContext) {}

pub type CLISubagentView = SubagentView;
pub type CLISubagentAction = SubagentAction;
pub type CLISubagentViewEvent = SubagentViewEvent;

pub struct SubagentView;

impl SubagentView {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        _block_id: BlockId,
        _action_model: ModelHandle<BlocklistAIActionModel>,
        _subagent_controller: ModelHandle<CLISubagentController>,
        _terminal_model: Arc<FairMutex<TerminalModel>>,
        _conversation_id: AIConversationId,
        _task_id: TaskId,
        _current_working_directory: Option<String>,
        _shell_launch_data: Option<ShellLaunchData>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn clear_all_selections(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn selected_text(&self, _ctx: &AppContext) -> Option<String> {
        None
    }
}

#[derive(Clone)]
pub enum SubagentViewEvent {
    TextSelected,
    CopiedEmptyText,
    #[cfg(windows)]
    WindowsCtrlC,
}

impl std::fmt::Debug for SubagentViewEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Event")
    }
}

impl Entity for SubagentView {
    type Event = SubagentViewEvent;
}

impl View for SubagentView {
    fn ui_name() -> &'static str {
        "SubagentView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

#[derive(Clone)]
pub enum SubagentAction {
    CopyCode(String),
    OpenCodeBlock(CodeSource),
    ExecuteBlockedAction,
    ExecuteAndAutoApprove,
    RejectBlockedAction { should_user_take_over: bool },
    TakeControlOfRunningCommand,
    ToggleAllowMenu,
    ToggleAlwaysAllowWriteToPty,
    ToggleAlwaysAllowReadFiles,
    DismissInput,
    SelectText,
    CopyDebugId(String),
    OpenFeedbackDocs,
}

impl std::fmt::Debug for SubagentAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Action")
    }
}

impl TypedActionView for SubagentView {
    type Action = SubagentAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}
