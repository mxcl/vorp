use warp_core::telemetry::{EnablementState, TelemetryEvent, TelemetryEventDesc};

#[derive(Debug)]
#[allow(dead_code)]
pub(super) enum CliTelemetryEvent {
    AgentRun {
        gui: bool,
        requested_mcp_servers: usize,
        has_environment: bool,
        task_id: Option<String>,
        harness: String,
    },
    AgentRunAmbient,
    AgentProfileList,
    AgentList,
    EnvironmentList,
    EnvironmentCreate,
    EnvironmentDelete,
    EnvironmentUpdate,
    EnvironmentGet,
    EnvironmentImageList,
    MCPList,
    ModelList,
    TaskList,
    TaskGet,
    ConversationGet,
    RunConversationGet,
    RunMessageWatch {
        harness: &'static str,
    },
    RunMessageSend {
        harness: &'static str,
    },
    RunMessageList {
        harness: &'static str,
    },
    RunMessageRead {
        harness: &'static str,
    },
    RunMessageMarkDelivered {
        harness: &'static str,
    },
    Login,
    Logout,
    Whoami,
    ProviderSetup,
    ProviderList,
    IntegrationCreate,
    IntegrationUpdate,
    IntegrationList,
    ArtifactUpload,
    ArtifactGet,
    ArtifactDownload,
    ScheduleCreate,
    ScheduleList,
    ScheduleGet,
    SchedulePause,
    ScheduleUnpause,
    ScheduleUpdate,
    ScheduleDelete,
    SecretCreate,
    SecretDelete,
    SecretUpdate,
    SecretList,
    FederateIssueToken,
    FederateIssueGcpToken,
    HarnessSupportPing,
    HarnessSupportReportArtifact {
        artifact_type: &'static str,
    },
    HarnessSupportNotifyUser,
    HarnessSupportFinishTask {
        success: bool,
    },
}

impl TelemetryEvent for CliTelemetryEvent {
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

warp_core::register_telemetry_event!(CliTelemetryEvent);
