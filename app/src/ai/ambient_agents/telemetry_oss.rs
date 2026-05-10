use crate::server::ids::ServerId;
use warp_core::telemetry::{EnablementState, TelemetryEvent, TelemetryEventDesc};

#[derive(Clone, Copy, Debug)]
pub enum CloudModeEntryPoint {
    NewTab,
    LocalSession,
    OzLaunchModal,
    EntryBlock,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum CloudAgentTelemetryEvent {
    EnteredCloudMode {
        entry_point: CloudModeEntryPoint,
    },
    EnvironmentSelectorOpened,
    EnvironmentSelected {
        environment_id: Option<ServerId>,
    },
    OpenedEnvironmentManagementPane,
    EnvironmentCreated,
    EnvironmentUpdated {
        environment_id: Option<ServerId>,
    },
    EnvironmentDeleted {
        environment_id: Option<ServerId>,
    },
    ImageSuggested {
        image: String,
        needs_custom_image: bool,
    },
    ImageSuggestionFailed {
        error: String,
    },
    LaunchedAgentFromEnvironmentForm,
    GitHubAuthFromEnvironmentForm,
    DispatchFailed {
        error: String,
    },
}

impl TelemetryEvent for CloudAgentTelemetryEvent {
    fn name(&self) -> &'static str {
        "TelemetryDisabled"
    }

    fn payload(&self) -> Option<serde_json::Value> {
        None
    }

    fn description(&self) -> &'static str {
        ""
    }

    fn enablement_state(&self) -> EnablementState {
        EnablementState::Always
    }

    fn contains_ugc(&self) -> bool {
        false
    }

    fn event_descs() -> impl Iterator<Item = Box<dyn TelemetryEventDesc>> {
        std::iter::empty()
    }
}

warp_core::register_telemetry_event!(CloudAgentTelemetryEvent);
