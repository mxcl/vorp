use std::sync::Arc;

use pathfinder_color::ColorU;
use warp_core::ui::{appearance::Appearance, theme::Fill};
use warpui::{
    elements::Empty, AppContext, Element, Entity, SingletonEntity, TypedActionView, View,
    ViewContext,
};

use crate::{
    terminal::input::MenuPositioningProvider, view_components::action_button::ActionButtonTheme,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Host {
    Warp,
    SelfHosted { slug: String },
}

impl Host {
    pub fn worker_host_value(&self) -> Option<String> {
        match self {
            Host::Warp => Some("warp".to_string()),
            Host::SelfHosted { slug } => Some(slug.clone()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HostSelectorAction {
    ToggleMenu,
    SelectHost(Host),
}

pub enum HostSelectorEvent {
    MenuVisibilityChanged { open: bool },
    HostSelected,
}

pub struct HostSelector {
    selected: Host,
    default_host: Option<Host>,
}

impl HostSelector {
    pub fn new(
        _menu_positioning_provider: Arc<dyn MenuPositioningProvider>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            selected: Host::Warp,
            default_host: None,
        }
    }

    pub fn is_menu_open(&self) -> bool {
        false
    }

    pub fn has_default_host(&self) -> bool {
        self.default_host.is_some()
    }

    pub fn selected(&self) -> &Host {
        &self.selected
    }

    pub fn set_default_host(&mut self, slug: String, _ctx: &mut ViewContext<Self>) {
        let host = Host::SelfHosted { slug };
        self.selected = host.clone();
        self.default_host = Some(host);
    }

    pub fn open_menu(&mut self, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for HostSelector {
    type Event = HostSelectorEvent;
}

impl TypedActionView for HostSelector {
    type Action = HostSelectorAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            HostSelectorAction::ToggleMenu => {}
            HostSelectorAction::SelectHost(host) => {
                self.selected = host.clone();
                ctx.emit(HostSelectorEvent::HostSelected);
            }
        }
    }
}

impl View for HostSelector {
    fn ui_name() -> &'static str {
        "HostSelector"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

#[derive(Clone, Copy)]
pub struct NakedHeaderButtonTheme;

impl ActionButtonTheme for NakedHeaderButtonTheme {
    fn background(&self, _hovered: bool, _appearance: &Appearance) -> Option<Fill> {
        None
    }

    fn text_color(
        &self,
        _hovered: bool,
        background: Option<Fill>,
        appearance: &Appearance,
    ) -> ColorU {
        appearance
            .theme()
            .sub_text_color(background.unwrap_or(appearance.theme().background()))
            .into_solid()
    }

    fn border(&self, _appearance: &Appearance) -> Option<ColorU> {
        None
    }
}
