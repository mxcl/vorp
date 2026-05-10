use warp_core::telemetry::{EnablementState, TelemetryEvent, TelemetryEventDesc};

use crate::tab_configs::session_config::SessionType;

#[derive(Clone, Copy, Debug)]
pub enum ExistingTabConfigOpenMode {
    Direct,
    ParamsModal,
}

#[derive(Clone, Copy, Debug)]
pub enum NewWorktreeConfigOpenSource {
    Submenu,
    NewWorktreeModal,
}

#[derive(Clone, Copy, Debug)]
pub enum WorktreeBranchNamingMode {
    Auto,
    Manual,
}

#[derive(Clone, Copy, Debug)]
pub enum GuidedModalSessionType {
    Terminal,
    Oz,
    CliAgent,
}

impl From<&SessionType> for GuidedModalSessionType {
    fn from(value: &SessionType) -> Self {
        match value {
            SessionType::Terminal => Self::Terminal,
            SessionType::Oz => Self::Oz,
            SessionType::CliAgent(_) => Self::CliAgent,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum TabConfigsTelemetryEvent {
    MenuCreateNewTabConfigClicked,
    ExistingConfigOpened {
        open_mode: ExistingTabConfigOpenMode,
        is_worktree_config: bool,
    },
    NewWorktreeConfigOpened {
        source: NewWorktreeConfigOpenSource,
        naming_mode: WorktreeBranchNamingMode,
    },
    GuidedModalOpened,
    GuidedModalSubmitted {
        session_type: GuidedModalSessionType,
        enable_worktree: bool,
        autogenerate_worktree_branch_name: bool,
    },
}

impl TelemetryEvent for TabConfigsTelemetryEvent {
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

warp_core::register_telemetry_event!(TabConfigsTelemetryEvent);
