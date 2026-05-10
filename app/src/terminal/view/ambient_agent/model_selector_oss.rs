use std::sync::Arc;

use warpui::{
    elements::Empty, AppContext, Element, Entity, EntityId, SingletonEntity, TypedActionView, View,
    ViewContext,
};

use crate::{ai::llms::LLMId, terminal::input::MenuPositioningProvider};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModelSelectorAction {
    ToggleMenu,
    SelectModel(LLMId),
}

pub enum ModelSelectorEvent {
    MenuVisibilityChanged { open: bool },
}

pub struct ModelSelector;

impl ModelSelector {
    pub fn new(
        _menu_positioning_provider: Arc<dyn MenuPositioningProvider>,
        _terminal_view_id: EntityId,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn is_menu_open(&self) -> bool {
        false
    }

    pub fn open_menu(&mut self, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for ModelSelector {
    type Event = ModelSelectorEvent;
}

impl TypedActionView for ModelSelector {
    type Action = ModelSelectorAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for ModelSelector {
    fn ui_name() -> &'static str {
        "ModelSelector"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
