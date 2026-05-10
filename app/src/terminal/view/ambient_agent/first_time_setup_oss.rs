use warpui::{elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext};

#[derive(Debug, Clone)]
pub enum FirstTimeCloudAgentSetupViewEvent {
    Cancelled,
    EnvironmentCreated,
}

#[derive(Debug, Clone)]
pub enum FirstTimeCloudAgentSetupAction {
    Noop,
}

pub struct FirstTimeCloudAgentSetupView;

impl FirstTimeCloudAgentSetupView {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn reset_form(&mut self, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for FirstTimeCloudAgentSetupView {
    type Event = FirstTimeCloudAgentSetupViewEvent;
}

impl TypedActionView for FirstTimeCloudAgentSetupView {
    type Action = FirstTimeCloudAgentSetupAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for FirstTimeCloudAgentSetupView {
    fn ui_name() -> &'static str {
        "FirstTimeCloudAgentSetupView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
