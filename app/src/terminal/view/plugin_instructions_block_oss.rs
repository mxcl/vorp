use warpui::elements::Empty;
use warpui::{AppContext, Element, Entity, TypedActionView, View, ViewContext};

use crate::terminal::cli_agent_sessions::plugin_manager::PluginInstructions;
use crate::terminal::CLIAgent;

pub(crate) struct PluginInstructionsBlock;

impl PluginInstructionsBlock {
    pub fn new(
        _instructions: &'static PluginInstructions,
        _agent: CLIAgent,
        _custom_command_prefix: Option<String>,
        _is_remote_session: bool,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }
}

impl Entity for PluginInstructionsBlock {
    type Event = PluginInstructionsBlockEvent;
}

impl View for PluginInstructionsBlock {
    fn ui_name() -> &'static str {
        "PluginInstructionsBlock"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for PluginInstructionsBlock {
    type Action = PluginInstructionsBlockAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            PluginInstructionsBlockAction::Close => {
                ctx.emit(PluginInstructionsBlockEvent::Close);
                ctx.notify();
            }
            PluginInstructionsBlockAction::CopyCommand(_) => {}
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum PluginInstructionsBlockAction {
    Close,
    CopyCommand(usize),
}

pub(crate) enum PluginInstructionsBlockEvent {
    Close,
}
