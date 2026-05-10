use ai::skills::{SkillProvider, SkillReference, SkillScope};
use serde::{Deserialize, Serialize};
use warp_core::telemetry::{EnablementState, TelemetryEvent, TelemetryEventDesc};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SkillOpenOrigin {
    ReadSkill,
    ReadFiles,
    EditFiles,
    OpenSkillCommand,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum SkillTelemetryEvent {
    Read {
        reference: SkillReference,
        name: Option<String>,
        scope: Option<SkillScope>,
        provider: Option<SkillProvider>,
        error: bool,
    },
    Opened {
        reference: SkillReference,
        name: Option<String>,
        origin: SkillOpenOrigin,
    },
}

impl TelemetryEvent for SkillTelemetryEvent {
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
        true
    }

    fn event_descs() -> impl Iterator<Item = Box<dyn TelemetryEventDesc>> {
        std::iter::empty()
    }
}

warp_core::register_telemetry_event!(SkillTelemetryEvent);
