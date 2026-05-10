use serde_json::{json, Value};
use strum_macros::EnumDiscriminants;
use strum_macros::EnumIter;
use warp_core::telemetry::{EnablementState, TelemetryEvent, TelemetryEventDesc};

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
#[cfg_attr(not(windows), expect(dead_code))]
pub(super) enum AntivirusInfoTelemetryEvent {
    AntivirusDetected { name: String },
}

impl TelemetryEvent for AntivirusInfoTelemetryEvent {
    #[cfg(feature = "oss_release")]
    fn name(&self) -> &'static str {
        "TelemetryDisabled"
    }

    #[cfg(not(feature = "oss_release"))]
    fn name(&self) -> &'static str {
        AntivirusInfoTelemetryEventDiscriminants::from(self).name()
    }

    fn payload(&self) -> Option<Value> {
        match self {
            AntivirusInfoTelemetryEvent::AntivirusDetected { name } => Some(json!({
                "antivirus_name": name,
            })),
        }
    }

    #[cfg(feature = "oss_release")]
    fn description(&self) -> &'static str {
        ""
    }

    #[cfg(not(feature = "oss_release"))]
    fn description(&self) -> &'static str {
        AntivirusInfoTelemetryEventDiscriminants::from(self).description()
    }

    fn enablement_state(&self) -> EnablementState {
        AntivirusInfoTelemetryEventDiscriminants::from(self).enablement_state()
    }

    fn contains_ugc(&self) -> bool {
        match self {
            AntivirusInfoTelemetryEvent::AntivirusDetected { .. } => false,
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

impl TelemetryEventDesc for AntivirusInfoTelemetryEventDiscriminants {
    #[cfg(feature = "oss_release")]
    fn name(&self) -> &'static str {
        "TelemetryDisabled"
    }

    #[cfg(not(feature = "oss_release"))]
    fn name(&self) -> &'static str {
        match self {
            AntivirusInfoTelemetryEventDiscriminants::AntivirusDetected => {
                "Identified Antivirus Software"
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
            AntivirusInfoTelemetryEventDiscriminants::AntivirusDetected => {
                "Identified running antivirus software on the user's machine"
            }
        }
    }

    fn enablement_state(&self) -> EnablementState {
        match self {
            AntivirusInfoTelemetryEventDiscriminants::AntivirusDetected => EnablementState::Always,
        }
    }
}

warp_core::register_telemetry_event!(AntivirusInfoTelemetryEvent);
