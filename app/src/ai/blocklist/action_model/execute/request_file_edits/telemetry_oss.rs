use ai::diff_validation::DiffMatchFailures;
use warp_core::telemetry::{EnablementState, TelemetryEvent, TelemetryEventDesc};

use crate::ai::{agent::AIIdentifiers, blocklist::RequestedEditResolution};

#[derive(Debug)]
#[allow(dead_code)]
pub enum RequestFileEditsTelemetryEvent {
    EditResolved(EditResolvedEvent),
    EditAcceptClicked(EditAcceptClickedEvent),
    EditAcceptAndContinueClicked(EditAcceptAndContinueClickedEvent),
    DiffMatchFailed(DiffMatchFailedEvent),
    DiffInvalidFile(DiffInvalidFileEvent),
    EditReceived(EditReceivedEvent),
    MissingLineNumbers(MissingLineNumbersEvent),
    MalformedFinalLineProxy(MalformedFinalLineProxyEvent),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct EditResolvedEvent {
    pub identifiers: AIIdentifiers,
    pub response: RequestedEditResolution,
    pub stats: EditStats,
    pub passive_diff: bool,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct EditAcceptClickedEvent {
    pub identifiers: AIIdentifiers,
    pub passive_diff: bool,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct EditAcceptAndContinueClickedEvent {
    pub identifiers: AIIdentifiers,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct EditStats {
    pub files_edited: usize,
    pub lines_added: usize,
    pub lines_removed: usize,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DiffMatchFailedEvent {
    pub identifiers: AIIdentifiers,
    pub failures: DiffMatchFailures,
    pub passive_diff: bool,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DiffInvalidFileEvent {
    pub identifiers: AIIdentifiers,
    pub count: usize,
    pub passive_diff: bool,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct EditReceivedEvent {
    pub identifiers: AIIdentifiers,
    pub unique_files: usize,
    pub diffs: usize,
    pub passive_diff: bool,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct MissingLineNumbersEvent {
    pub identifiers: AIIdentifiers,
    pub count: u8,
    pub passive_diff: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum RequestFileEditsFormatKind {
    StrReplace,
    V4A,
    Mixed,
    Unknown,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct MalformedFinalLineProxyEvent {
    pub identifiers: AIIdentifiers,
    pub file_count: usize,
    pub edited_file_count: usize,
    pub correction_count: usize,
    pub edited_correction_count: usize,
    pub unedited_correction_count: usize,
    pub format_kind: RequestFileEditsFormatKind,
    pub passive_diff: bool,
}

impl TelemetryEvent for RequestFileEditsTelemetryEvent {
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

warp_core::register_telemetry_event!(RequestFileEditsTelemetryEvent);
