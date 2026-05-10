//! This module contains all code relevant to Agent Predict within Warp.
//!
//! Agent Predict attempts to predict the next action the user will take in Warp.

#[cfg(not(feature = "oss_release"))]
pub(crate) mod generate_ai_input_suggestions;
#[cfg(feature = "oss_release")]
#[path = "generate_ai_input_suggestions_oss.rs"]
pub(crate) mod generate_ai_input_suggestions;
#[cfg(not(feature = "oss_release"))]
pub(crate) mod generate_am_query_suggestions;
#[cfg(feature = "oss_release")]
#[path = "generate_am_query_suggestions_oss.rs"]
pub(crate) mod generate_am_query_suggestions;
#[cfg(not(feature = "oss_release"))]
pub mod next_command_model;
#[cfg(feature = "oss_release")]
#[path = "next_command_model_oss.rs"]
pub mod next_command_model;
#[cfg(not(feature = "oss_release"))]
pub(crate) mod predict_am_queries;
#[cfg(feature = "oss_release")]
#[path = "predict_am_queries_oss.rs"]
pub(crate) mod predict_am_queries;
#[cfg(not(feature = "oss_release"))]
pub mod prompt_suggestions;
#[cfg(feature = "oss_release")]
#[path = "prompt_suggestions_oss.rs"]
pub mod prompt_suggestions;
