use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, TypedActionView, View, ViewContext};

#[derive(Clone, Copy, Debug)]
pub enum PasteAuthTokenModalAction {
    Confirm,
    Cancel,
    PasteIntoEditor,
}

#[derive(Clone, Debug)]
pub enum PasteAuthTokenModalEvent {
    Cancelled,
}

pub struct PasteAuthTokenModalView;

impl PasteAuthTokenModalView {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }
}

impl Entity for PasteAuthTokenModalView {
    type Event = PasteAuthTokenModalEvent;
}

impl View for PasteAuthTokenModalView {
    fn ui_name() -> &'static str {
        "PasteAuthTokenModalView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for PasteAuthTokenModalView {
    type Action = PasteAuthTokenModalAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        if matches!(action, PasteAuthTokenModalAction::Cancel) {
            ctx.emit(PasteAuthTokenModalEvent::Cancelled);
        }
    }
}
