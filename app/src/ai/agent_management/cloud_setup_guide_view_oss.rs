use crate::workflows::WorkflowType;
use serde::Serialize;
use warpui::{elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext};

pub struct CloudSetupGuideView;

#[derive(Debug, Clone)]
pub enum CloudSetupGuideAction {
    Noop,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub enum SetupGuideDocs {
    Main,
    Environment,
    Integration,
}

pub enum CloudSetupGuideEvent {
    OpenNewTabAndInsertWorkflow(WorkflowType),
}

impl CloudSetupGuideView {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }
}

impl Entity for CloudSetupGuideView {
    type Event = CloudSetupGuideEvent;
}

impl View for CloudSetupGuideView {
    fn ui_name() -> &'static str {
        "CloudSetupGuideView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for CloudSetupGuideView {
    type Action = CloudSetupGuideAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}
