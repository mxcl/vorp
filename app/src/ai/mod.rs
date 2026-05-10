//! This module should houses all horizontal/cross-cutting AI functionality throughout
//! Warp (including Agent Mode).
//!
//! The side panel Warp AI implementation lives in `super::ai_assistant`.
pub(crate) mod active_agent_views_model;
pub(crate) mod agent;
pub(crate) mod agent_conversations_model;
pub(crate) mod agent_events;
pub(crate) mod agent_management;
#[cfg(not(feature = "oss_release"))]
pub(crate) mod agent_tips;
#[cfg(feature = "oss_release")]
#[path = "agent_tips_oss.rs"]
pub(crate) mod agent_tips;
#[cfg(not(feature = "oss_release"))]
pub(crate) mod ai_document_view;
#[cfg(feature = "oss_release")]
#[path = "ai_document_view_oss.rs"]
pub(crate) mod ai_document_view;
pub mod ambient_agents;
pub(crate) mod artifact_download;
pub mod artifacts;
pub(crate) mod attachment_utils;
#[cfg(not(target_family = "wasm"))]
pub mod aws_credentials;
pub(crate) mod block_context;
pub(crate) mod blocklist;
pub mod control_code_parser;
#[cfg(not(feature = "oss_release"))]
pub(crate) mod conversation_details_panel;
#[cfg(feature = "oss_release")]
#[path = "conversation_details_panel_oss.rs"]
pub(crate) mod conversation_details_panel;
pub(crate) mod conversation_navigation;
pub(crate) mod conversation_status_ui;
pub(crate) mod conversation_utils;
pub(crate) mod document;
pub(crate) mod get_relevant_files;
pub mod harness_availability;
pub(crate) mod harness_display;
#[cfg(not(feature = "oss_release"))]
pub(crate) mod llms;
#[cfg(feature = "oss_release")]
#[path = "llms_oss.rs"]
pub(crate) mod llms;
pub mod onboarding;
pub(crate) mod persisted_workspace;
pub(crate) mod predict;
pub mod request_usage_model;
pub(crate) mod restored_conversations;
pub(crate) mod skills;
#[cfg(not(feature = "oss_release"))]
pub(crate) mod voice;
#[cfg(feature = "oss_release")]
#[path = "voice_oss.rs"]
pub(crate) mod voice;
pub use agent_tips::*;
pub use request_usage_model::*;
use warpui::AppContext;
#[cfg(all(not(target_family = "wasm"), not(feature = "oss_release")))]
pub mod agent_sdk;
#[cfg(not(feature = "oss_release"))]
pub mod cloud_agent_config;
#[cfg(not(feature = "oss_release"))]
pub mod cloud_agent_settings;
pub mod cloud_environments;
pub mod execution_profiles;
pub mod facts;
pub(crate) mod generate_block_title;
pub(crate) mod generate_code_review_content;
pub(crate) mod loading;
#[cfg(not(feature = "oss_release"))]
pub mod mcp;
#[cfg(feature = "oss_release")]
#[path = "mcp_oss.rs"]
pub mod mcp;
pub mod outline;

pub(crate) use ai::paths;

pub fn init(app: &mut AppContext) {
    blocklist::keyboard_navigable_buttons::init(app);
    blocklist::block::number_shortcut_buttons::init(app);
    blocklist::toggleable_items::init(app);
    blocklist::suggested_agent_mode_workflow_modal::init(app);
    blocklist::suggested_rule_modal::init(app);
    ai_document_view::init(app);
    conversation_details_panel::init(app);
    agent_management::init(app);
}
