use std::sync::Arc;

use warp_cli::agent::Harness;
use warpui::{
    elements::Empty, AppContext, Element, Entity, ModelHandle, SingletonEntity, TypedActionView,
    View, ViewContext,
};

use crate::{
    terminal::{input::MenuPositioningProvider, view::ambient_agent::AmbientAgentViewModel},
    view_components::action_button::ActionButtonTheme,
};

#[derive(Clone, Debug, PartialEq)]
pub enum HarnessSelectorAction {
    ToggleMenu,
    SelectHarness(Harness),
}

pub enum HarnessSelectorEvent {
    MenuVisibilityChanged { open: bool },
}

pub struct HarnessSelector;

impl HarnessSelector {
    pub fn new(
        _menu_positioning_provider: Arc<dyn MenuPositioningProvider>,
        _ambient_agent_model: ModelHandle<AmbientAgentViewModel>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn is_menu_open(&self) -> bool {
        false
    }

    pub fn open_menu(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn set_button_theme<T>(&self, _theme: T, _ctx: &mut ViewContext<Self>)
    where
        T: ActionButtonTheme + Clone + 'static,
    {
    }
}

impl Entity for HarnessSelector {
    type Event = HarnessSelectorEvent;
}

impl TypedActionView for HarnessSelector {
    type Action = HarnessSelectorAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for HarnessSelector {
    fn ui_name() -> &'static str {
        "HarnessSelector"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
