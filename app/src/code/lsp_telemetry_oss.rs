use warp_core::telemetry::{EnablementState, TelemetryEvent, TelemetryEventDesc};

#[derive(Clone, Debug)]
pub enum LspEnablementSource {
    InitFlow,
    FooterButton,
    Settings,
}

#[derive(Clone, Debug)]
pub enum LspControlActionType {
    OpenLogs,
    Restart,
    Stop,
    Start,
    RestartAll,
    StopAll,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum LspTelemetryEvent {
    ServerEnabled {
        server_type: String,
        source: LspEnablementSource,
        needed_install: bool,
    },
    ServerEnablementSkipped,
    ServerInstallCompleted {
        server_type: String,
        success: bool,
    },
    ServerRemoved {
        server_type: String,
        source: LspEnablementSource,
    },
    HoverShown {
        server_type: String,
        had_content: bool,
        had_diagnostics: bool,
    },
    GotoDefinition {
        server_type: String,
        had_result: bool,
    },
    FindReferencesShown {
        server_type: String,
        num_references: usize,
    },
    ControlAction {
        action: LspControlActionType,
        server_type: Option<String>,
    },
    ServerStarted {
        server_type: String,
    },
    ServerFailed {
        server_type: String,
    },
}

impl TelemetryEvent for LspTelemetryEvent {
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

warp_core::register_telemetry_event!(LspTelemetryEvent);
