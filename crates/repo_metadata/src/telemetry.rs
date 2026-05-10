#[cfg(not(feature = "oss_release"))]
use serde_json::json;
use serde_json::Value;
use strum_macros::{EnumDiscriminants, EnumIter};
use warp_core::{
    register_telemetry_event,
    telemetry::{EnablementState, TelemetryEvent, TelemetryEventDesc},
};

#[derive(Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
pub enum RepoMetadataTelemetryEvent {
    BuildTreeFailed { error: String },
}

impl TelemetryEvent for RepoMetadataTelemetryEvent {
    #[cfg(feature = "oss_release")]
    fn name(&self) -> &'static str {
        "TelemetryDisabled"
    }

    #[cfg(not(feature = "oss_release"))]
    fn name(&self) -> &'static str {
        RepoMetadataTelemetryEventDiscriminants::from(self).name()
    }

    #[cfg(feature = "oss_release")]
    fn description(&self) -> &'static str {
        ""
    }

    #[cfg(not(feature = "oss_release"))]
    fn description(&self) -> &'static str {
        RepoMetadataTelemetryEventDiscriminants::from(self).description()
    }

    #[cfg(feature = "oss_release")]
    fn enablement_state(&self) -> EnablementState {
        EnablementState::Always
    }

    #[cfg(not(feature = "oss_release"))]
    fn enablement_state(&self) -> EnablementState {
        RepoMetadataTelemetryEventDiscriminants::from(self).enablement_state()
    }

    #[cfg(feature = "oss_release")]
    fn payload(&self) -> Option<Value> {
        None
    }

    #[cfg(not(feature = "oss_release"))]
    fn payload(&self) -> Option<Value> {
        match self {
            Self::BuildTreeFailed { error } => Some(json!({
                "error": error
            })),
        }
    }

    #[cfg(feature = "oss_release")]
    fn contains_ugc(&self) -> bool {
        false
    }

    #[cfg(not(feature = "oss_release"))]
    fn contains_ugc(&self) -> bool {
        match self {
            Self::BuildTreeFailed { .. } => false,
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

impl TelemetryEventDesc for RepoMetadataTelemetryEventDiscriminants {
    #[cfg(feature = "oss_release")]
    fn name(&self) -> &'static str {
        "TelemetryDisabled"
    }

    #[cfg(not(feature = "oss_release"))]
    fn name(&self) -> &'static str {
        match self {
            Self::BuildTreeFailed => "RepoMetadata.BuildTree.Failed",
        }
    }

    #[cfg(feature = "oss_release")]
    fn description(&self) -> &'static str {
        ""
    }

    #[cfg(not(feature = "oss_release"))]
    fn description(&self) -> &'static str {
        match self {
            Self::BuildTreeFailed => "Failed to build file tree for repo metadata",
        }
    }

    fn enablement_state(&self) -> EnablementState {
        match self {
            Self::BuildTreeFailed => EnablementState::Always,
        }
    }
}

register_telemetry_event!(RepoMetadataTelemetryEvent);
