use serde_json::Value;
use strum_macros::EnumDiscriminants;
use strum_macros::EnumIter;
use warp_core::telemetry::{EnablementState, TelemetryEvent, TelemetryEventDesc};

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
pub enum SettingsTelemetryEvent {
    EnvironmentsPageOpened,
}

impl TelemetryEvent for SettingsTelemetryEvent {
    #[cfg(feature = "oss_release")]
    fn name(&self) -> &'static str {
        "TelemetryDisabled"
    }

    #[cfg(not(feature = "oss_release"))]
    fn name(&self) -> &'static str {
        SettingsTelemetryEventDiscriminants::from(self).name()
    }

    fn payload(&self) -> Option<Value> {
        None
    }

    #[cfg(feature = "oss_release")]
    fn description(&self) -> &'static str {
        ""
    }

    #[cfg(not(feature = "oss_release"))]
    fn description(&self) -> &'static str {
        SettingsTelemetryEventDiscriminants::from(self).description()
    }

    fn enablement_state(&self) -> EnablementState {
        SettingsTelemetryEventDiscriminants::from(self).enablement_state()
    }

    fn contains_ugc(&self) -> bool {
        match self {
            SettingsTelemetryEvent::EnvironmentsPageOpened => false,
        }
    }

    #[cfg(not(feature = "oss_release"))]
    fn event_descs() -> impl Iterator<Item = Box<dyn TelemetryEventDesc>> {
        warp_core::telemetry::enum_events::<Self>()
    }

    #[cfg(feature = "oss_release")]
    fn event_descs() -> impl Iterator<Item = Box<dyn TelemetryEventDesc>> {
        std::iter::empty()
    }
}

impl TelemetryEventDesc for SettingsTelemetryEventDiscriminants {
    #[cfg(feature = "oss_release")]
    fn name(&self) -> &'static str {
        "TelemetryDisabled"
    }

    #[cfg(not(feature = "oss_release"))]
    fn name(&self) -> &'static str {
        match self {
            SettingsTelemetryEventDiscriminants::EnvironmentsPageOpened => {
                "Settings.Environments.PageOpened"
            }
        }
    }

    #[cfg(feature = "oss_release")]
    fn description(&self) -> &'static str {
        ""
    }

    #[cfg(not(feature = "oss_release"))]
    fn description(&self) -> &'static str {
        match self {
            SettingsTelemetryEventDiscriminants::EnvironmentsPageOpened => {
                "User opened the Environments settings page"
            }
        }
    }

    fn enablement_state(&self) -> EnablementState {
        match self {
            SettingsTelemetryEventDiscriminants::EnvironmentsPageOpened => EnablementState::Always,
        }
    }
}

warp_core::register_telemetry_event!(SettingsTelemetryEvent);
