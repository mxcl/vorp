#[cfg(not(feature = "oss_release"))]
mod agent_management_model;
#[cfg(feature = "oss_release")]
#[path = "agent_management_model_oss.rs"]
mod agent_management_model;
#[cfg(not(feature = "oss_release"))]
pub(crate) mod agent_type_selector;
#[cfg(not(feature = "oss_release"))]
pub(crate) mod details_action_buttons;
#[cfg(not(feature = "oss_release"))]
pub(crate) mod notifications;
#[cfg(feature = "oss_release")]
#[path = "notifications_oss.rs"]
pub(crate) mod notifications;

#[cfg(not(feature = "oss_release"))]
pub(crate) mod cloud_setup_guide_view;
#[cfg(feature = "oss_release")]
#[path = "cloud_setup_guide_view_oss.rs"]
pub(crate) mod cloud_setup_guide_view;
#[cfg(not(feature = "oss_release"))]
pub(crate) mod telemetry;
#[cfg(feature = "oss_release")]
#[path = "telemetry_oss.rs"]
pub(crate) mod telemetry;
#[cfg(not(feature = "oss_release"))]
pub(crate) mod view;
#[cfg(feature = "oss_release")]
#[path = "view_oss.rs"]
pub(crate) mod view;

pub(crate) use agent_management_model::{AgentManagementEvent, AgentNotificationsModel};

pub fn init(app: &mut warpui::AppContext) {
    view::init(app);
    #[cfg(not(feature = "oss_release"))]
    agent_type_selector::init(app);
    notifications::view::NotificationMailboxView::init(app);
}
