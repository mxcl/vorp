use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, TypedActionView, View, ViewContext};

pub fn init(_app: &mut AppContext) {}

#[derive(Clone, Debug)]
pub enum OpenWarpLaunchModalAction {
    Close,
    VisitRepo,
}

#[derive(Clone, Debug)]
pub enum OpenWarpLaunchModalEvent {
    Close,
}

pub struct OpenWarpLaunchModal;

impl OpenWarpLaunchModal {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }
}

impl Entity for OpenWarpLaunchModal {
    type Event = OpenWarpLaunchModalEvent;
}

impl View for OpenWarpLaunchModal {
    fn ui_name() -> &'static str {
        "LaunchModal"
    }

    fn render(&self, _ctx: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for OpenWarpLaunchModal {
    type Action = OpenWarpLaunchModalAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        if matches!(action, OpenWarpLaunchModalAction::Close) {
            ctx.emit(OpenWarpLaunchModalEvent::Close);
        }
    }
}
