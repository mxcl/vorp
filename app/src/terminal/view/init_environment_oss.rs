pub mod mode_selector {
    use warpui::{
        elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum EnvironmentSetupMode {
        RemoteGitHub,
        LocalRepositories,
    }

    #[derive(Debug, Clone)]
    pub enum EnvironmentSetupModeSelectorAction {
        Noop,
    }

    #[derive(Debug)]
    pub enum EnvironmentSetupModeSelectorEvent {
        Selected(EnvironmentSetupMode),
        Dismissed,
    }

    pub struct EnvironmentSetupModeSelector;

    pub fn init(_app: &mut AppContext) {}

    impl EnvironmentSetupModeSelector {
        pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
            Self
        }
    }

    impl Entity for EnvironmentSetupModeSelector {
        type Event = EnvironmentSetupModeSelectorEvent;
    }

    impl TypedActionView for EnvironmentSetupModeSelector {
        type Action = EnvironmentSetupModeSelectorAction;

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }

    impl View for EnvironmentSetupModeSelector {
        fn ui_name() -> &'static str {
            "EnvironmentSetupModeSelector"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }
}

use warpui::{elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext};

#[derive(Debug, Clone)]
pub enum InitEnvironmentBlockAction {
    StartSetup,
    Skip,
}

#[derive(Debug)]
pub enum InitEnvironmentBlockEvent {
    StartSetup(Vec<String>, bool),
}

pub struct InitEnvironmentBlock;

impl InitEnvironmentBlock {
    pub fn try_steal_focus(&self, _ctx: &mut ViewContext<Self>) {}

    pub fn completed(&self) -> bool {
        true
    }

    pub fn handle_ctrl_c(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn new(
        _label: String,
        _repos: Vec<String>,
        _use_current_dir: bool,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }
}

impl Entity for InitEnvironmentBlock {
    type Event = InitEnvironmentBlockEvent;
}

impl TypedActionView for InitEnvironmentBlock {
    type Action = InitEnvironmentBlockAction;

    fn handle_action(&mut self, _action: &Self::Action, ctx: &mut ViewContext<Self>) {
        ctx.notify();
    }
}

impl View for InitEnvironmentBlock {
    fn ui_name() -> &'static str {
        "InitEnvironmentBlock"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
