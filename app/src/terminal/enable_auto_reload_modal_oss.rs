use crate::view_components::ToastFlavor;
use warpui::{AppContext, Element, Entity, View, ViewContext, elements::Empty};

pub struct EnableAutoReloadModal;

impl EnableAutoReloadModal {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn set_selected_denomination_by_credits(
        &mut self,
        _credits: i32,
        _ctx: &mut ViewContext<Self>,
    ) {
    }
}

#[derive(Clone, Debug)]
pub enum EnableAutoReloadModalEvent {
    Close,
    ShowToast {
        message: String,
        flavor: ToastFlavor,
    },
}

impl Entity for EnableAutoReloadModal {
    type Event = EnableAutoReloadModalEvent;
}

impl warpui::TypedActionView for EnableAutoReloadModal {
    type Action = ();
}

impl View for EnableAutoReloadModal {
    fn ui_name() -> &'static str {
        "EnableAutoReloadModal"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
