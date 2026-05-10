#[cfg(not(feature = "oss_release"))]
pub(crate) mod agent_input_footer;
#[cfg(feature = "oss_release")]
#[path = "agent_input_footer_oss.rs"]
pub(crate) mod agent_input_footer;
#[cfg(not(feature = "oss_release"))]
mod agent_message_bar;
#[cfg(feature = "oss_release")]
#[path = "agent_message_bar_oss.rs"]
mod agent_message_bar;
#[cfg(not(feature = "oss_release"))]
mod agent_view_block;
#[cfg(feature = "oss_release")]
#[path = "agent_view_block_oss.rs"]
mod agent_view_block;
#[cfg(not(feature = "oss_release"))]
pub mod child_agent_status_card;
#[cfg(feature = "oss_release")]
#[path = "child_agent_status_card_oss.rs"]
pub mod child_agent_status_card;
#[cfg(not(feature = "oss_release"))]
mod controller;
#[cfg(feature = "oss_release")]
#[path = "controller_oss.rs"]
mod controller;
#[cfg(not(feature = "oss_release"))]
mod ephemeral_message_model;
#[cfg(feature = "oss_release")]
#[path = "ephemeral_message_model_oss.rs"]
mod ephemeral_message_model;
#[cfg(not(feature = "oss_release"))]
mod inline_agent_view_header;
#[cfg(feature = "oss_release")]
#[path = "inline_agent_view_header_oss.rs"]
mod inline_agent_view_header;
// TODO: Move orchestration_conversation_links module import elsewhere.
pub(crate) mod orchestration_conversation_links;
#[cfg(not(feature = "oss_release"))]
pub mod orchestration_pill_bar;
#[cfg(feature = "oss_release")]
#[path = "orchestration_pill_bar_oss.rs"]
pub mod orchestration_pill_bar;
pub mod shortcuts;
#[cfg(not(feature = "oss_release"))]
mod zero_state_block;
#[cfg(feature = "oss_release")]
#[path = "zero_state_block_oss.rs"]
mod zero_state_block;

pub use agent_input_footer::*;
pub use agent_message_bar::*;
pub use agent_view_block::*;
pub use controller::*;
pub use ephemeral_message_model::*;
pub use inline_agent_view_header::*;
pub use orchestration_pill_bar::{render_orchestration_breadcrumbs, OrchestrationPillBar};
use warpui::fonts::Properties;
pub use zero_state_block::*;

use std::sync::LazyLock;

use pathfinder_color::ColorU;
use warp_core::ui::theme::Fill;
use warp_core::ui::{appearance::Appearance, color::blend::Blend};
use warpui::keymap::Keystroke;
use warpui::{AppContext, SingletonEntity};

use crate::view_components::action_button::ActionButtonTheme;

pub static ENTER_AGENT_VIEW_NEW_CONVERSATION_KEYSTROKE: LazyLock<Keystroke> = LazyLock::new(|| {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            Keystroke {
                cmd: true,
                key: "enter".to_owned(),
                ..Default::default()
            }
        } else {
            Keystroke {
                ctrl: true,
                shift: true,
                key: "enter".to_owned(),
                ..Default::default()
            }
        }
    }
});

pub static ENTER_CLOUD_AGENT_VIEW_NEW_CONVERSATION_KEYSTROKE: LazyLock<Keystroke> =
    LazyLock::new(|| {
        cfg_if::cfg_if! {
            if #[cfg(target_os = "macos")] {
                Keystroke {
                    cmd: true,
                    alt: true,
                    key: "enter".to_owned(),
                    ..Default::default()
                }
            } else {
                Keystroke {
                    ctrl: true,
                    alt: true,
                    key: "enter".to_owned(),
                    ..Default::default()
                }
            }
        }
    });

pub fn agent_view_bg_fill(app: &AppContext) -> Fill {
    let appearance = Appearance::as_ref(app);
    appearance.theme().surface_overlay_1()
}

pub fn agent_view_bg_color(app: &AppContext) -> ColorU {
    agent_view_bg_fill(app)
        .blend(&Appearance::as_ref(app).theme().background())
        .into_solid()
}

pub struct AgentViewHeaderTheme;

impl ActionButtonTheme for AgentViewHeaderTheme {
    fn background(&self, _: bool, _: &Appearance) -> Option<Fill> {
        None
    }

    fn text_color(
        &self,
        hovered: bool,
        background: Option<Fill>,
        appearance: &Appearance,
    ) -> ColorU {
        if hovered {
            appearance
                .theme()
                .main_text_color(background.unwrap_or(appearance.theme().background()))
                .into_solid()
        } else {
            appearance
                .theme()
                .sub_text_color(background.unwrap_or(appearance.theme().background()))
                .into_solid()
        }
    }

    fn font_properties(&self) -> Option<Properties> {
        Some(Properties::default())
    }

    fn keyboard_shortcut_background(&self, appearance: &Appearance) -> Option<ColorU> {
        Some(appearance.theme().surface_overlay_2().into_solid())
    }
}

pub struct AgentViewHeaderDisabledTheme;

impl ActionButtonTheme for AgentViewHeaderDisabledTheme {
    fn background(&self, _: bool, _: &Appearance) -> Option<Fill> {
        None
    }

    fn text_color(&self, _: bool, background: Option<Fill>, appearance: &Appearance) -> ColorU {
        appearance
            .theme()
            .disabled_text_color(background.unwrap_or(appearance.theme().background()))
            .into_solid()
    }

    fn keyboard_shortcut_background(&self, _: &Appearance) -> Option<ColorU> {
        None
    }

    fn font_properties(&self) -> Option<Properties> {
        Some(Properties::default())
    }
}
