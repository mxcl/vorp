pub(crate) mod action_sidecar;
#[cfg(not(feature = "oss_release"))]
pub mod branch_picker;
#[cfg(feature = "oss_release")]
#[path = "branch_picker_oss.rs"]
pub mod branch_picker;
#[cfg(not(feature = "oss_release"))]
pub mod new_worktree_modal;
#[cfg(feature = "oss_release")]
#[path = "new_worktree_modal_oss.rs"]
pub mod new_worktree_modal;
pub mod params_modal;
pub(crate) mod remove_confirmation_dialog;
#[cfg(not(feature = "oss_release"))]
pub mod repo_picker;
#[cfg(feature = "oss_release")]
#[path = "repo_picker_oss.rs"]
pub mod repo_picker;
pub mod session_config;
#[cfg(not(feature = "oss_release"))]
pub mod session_config_modal;
#[cfg(feature = "oss_release")]
#[path = "session_config_modal_oss.rs"]
pub mod session_config_modal;
pub mod session_config_rendering;
pub mod tab_config;
#[cfg(not(feature = "oss_release"))]
pub mod telemetry;
#[cfg(feature = "oss_release")]
#[path = "telemetry_oss.rs"]
pub mod telemetry;

use warp_core::ui::theme::Fill;

pub use new_worktree_modal::{NewWorktreeModal, NewWorktreeModalEvent};
pub use params_modal::{TabConfigParamsModal, TabConfigParamsModalEvent};
#[cfg(all(feature = "local_fs", not(feature = "oss_release")))]
pub(crate) use tab_config::build_worktree_config_toml;
pub use tab_config::{
    render_tab_config, TabConfig, TabConfigError, TabConfigParam, TabConfigParamType,
};

/// Optional visual overrides for BranchPicker / RepoPicker dropdowns.
pub struct PickerStyle {
    pub width: f32,
    pub background: Option<Fill>,
}
