use crate::workspace::WorkspaceAction;
use markdown_parser::FormattedTextFragment;
use warpui::keymap::Keystroke;
use warpui::{AppContext, Entity, ModelContext, SingletonEntity};

pub trait AITip: Clone {
    fn keystroke(&self, _app: &AppContext) -> Option<Keystroke> {
        None
    }

    fn link(&self) -> Option<String> {
        None
    }

    fn description(&self) -> &str;

    fn to_formatted_text(&self, _app: &AppContext) -> Vec<FormattedTextFragment> {
        Vec::new()
    }

    fn is_tip_applicable(
        &self,
        _current_working_directory: Option<&str>,
        _app: &AppContext,
    ) -> bool {
        false
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AgentTipKind {
    CodebaseContext,
    WarpDrive,
    General,
    Mcp,
    SlashCommands,
    Context,
    Code,
}

#[derive(Clone, Debug)]
pub struct AgentTip {
    pub description: String,
    pub link: Option<String>,
    pub binding_name: Option<&'static str>,
    pub action: Option<WorkspaceAction>,
    pub kind: AgentTipKind,
}

impl AITip for AgentTip {
    fn description(&self) -> &str {
        &self.description
    }
}

impl WorkspaceAction {
    pub fn display_text(&self) -> Option<String> {
        None
    }
}

pub fn get_agent_tips(_ctx: &AppContext) -> Vec<AgentTip> {
    Vec::new()
}

pub struct AITipModel<T: AITip> {
    current_tip: Option<T>,
}

impl<T: AITip + 'static> AITipModel<T> {
    pub fn new(_tips: Vec<T>) -> Self {
        Self { current_tip: None }
    }

    pub fn current_tip(&self) -> Option<&T> {
        self.current_tip.as_ref()
    }
}

impl<T: AITip + 'static> Entity for AITipModel<T> {
    type Event = ();
}

impl AITipModel<AgentTip> {
    pub fn new_for_agent_tips(_ctx: &AppContext) -> Self {
        Self { current_tip: None }
    }

    pub fn maybe_refresh_tip(
        &mut self,
        _current_working_directory: Option<&str>,
        _ctx: &mut ModelContext<Self>,
    ) {
        self.current_tip = None;
    }
}

impl SingletonEntity for AITipModel<AgentTip> {}

impl AITipModel<crate::terminal::view::ambient_agent::CloudModeTip> {
    pub fn maybe_refresh_tip(&mut self, _ctx: &mut ModelContext<Self>) {
        self.current_tip = None;
    }

    pub fn reset_cooldown(&mut self, _ctx: &mut ModelContext<Self>) {}
}
