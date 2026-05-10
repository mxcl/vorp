use crate::ai::agent_management::cloud_setup_guide_view::SetupGuideDocs;
use warp_core::telemetry::{EnablementState, TelemetryEvent, TelemetryEventDesc};

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum SetupGuideStep {
    VisitOz,
    CreateEnvironment,
    CreateEnvironmentCli,
    CreateSlackIntegration,
    CreateLinearIntegration,
}

#[derive(Clone, Copy, Debug)]
pub enum OpenedFrom {
    ManagementView,
    ConversationList,
    DetailsPanel,
}

#[derive(Clone, Copy, Debug)]
pub enum ArtifactType {
    Plan,
    Branch,
    PullRequest,
    File,
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum FilterType {
    Status,
    Source,
    CreatedOn,
    Creator,
    Owner,
    Harness,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum AgentManagementTelemetryEvent {
    ViewToggled {
        is_open: bool,
    },
    OpenSetupGuide,
    DismissSetupGuide,
    SpawnNewLocalAgent,
    SpawnNewCloudAgent,
    AgentTypeSelectorOpened,
    SetupGuideStepRun {
        step: SetupGuideStep,
    },
    SetupGuideStepCopy {
        step: SetupGuideStep,
    },
    SetupGuideDocsLink {
        docs: SetupGuideDocs,
    },
    ConversationOpened {
        conversation_id: String,
        opened_from: OpenedFrom,
    },
    CloudRunOpened {
        task_id: String,
        opened_from: OpenedFrom,
    },
    ArtifactClicked {
        artifact_type: ArtifactType,
    },
    FilterChanged {
        filter_type: FilterType,
    },
    DetailsViewed {
        item_id: String,
        viewed_from: OpenedFrom,
    },
    ConversationLinkCopied {
        conversation_id: String,
        copied_from: OpenedFrom,
    },
    SessionLinkCopied {
        task_id: String,
        copied_from: OpenedFrom,
    },
    TombstoneArtifactClicked {
        artifact_type: ArtifactType,
    },
    #[cfg(not(target_family = "wasm"))]
    TombstoneContinueLocally,
    #[cfg(not(target_family = "wasm"))]
    TombstoneContinueInCloud {
        task_id: String,
    },
    #[cfg(not(target_family = "wasm"))]
    DetailsPanelContinueLocally,
    #[cfg(not(target_family = "wasm"))]
    SlashCommandContinueLocally,
    #[cfg(target_family = "wasm")]
    TombstoneOpenInWarp,
    CloudRunCancelled {
        task_id: String,
    },
    ConversationForked {
        conversation_id: String,
    },
}

impl TelemetryEvent for AgentManagementTelemetryEvent {
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

warp_core::register_telemetry_event!(AgentManagementTelemetryEvent);
