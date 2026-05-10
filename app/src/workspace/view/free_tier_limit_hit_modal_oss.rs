use warpui::{elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext};

pub fn init(_app: &mut AppContext) {}

pub struct FreeTierLimitHitModal;

impl FreeTierLimitHitModal {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }
}

impl Entity for FreeTierLimitHitModal {
    type Event = FreeTierLimitHitModalEvent;
}

impl View for FreeTierLimitHitModal {
    fn ui_name() -> &'static str {
        "FreeTierLimitHitModal"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for FreeTierLimitHitModal {
    type Action = FreeTierLimitHitModalAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

#[derive(Copy, Clone, Debug)]
pub enum FreeTierLimitHitModalEvent {
    MaybeOpen,
    Close,
}

#[derive(Clone, Debug)]
pub enum FreeTierLimitHitModalAction {
    Close,
    OpenUpgrade,
    OpenUrl(String),
}
