#[cfg(not(feature = "oss_release"))]
pub mod schema;

#[cfg(feature = "oss_release")]
use http::StatusCode;
#[cfg(not(feature = "oss_release"))]
use warp_graphql::client::RequestOptions;
#[cfg(not(feature = "oss_release"))]
pub use warp_graphql::client::{get_request_context, get_user_facing_error_message, GraphQLError};

#[cfg(feature = "oss_release")]
#[derive(Debug, thiserror::Error)]
pub enum GraphQLError {
    #[error("staging access blocked")]
    StagingAccessBlocked,
    #[error("HTTP error {status}")]
    HttpError { status: StatusCode },
}

#[cfg(feature = "oss_release")]
pub fn get_user_facing_error_message<T>(_error: T) -> String {
    "Server GraphQL APIs are not available in this build".to_string()
}

#[cfg(feature = "oss_release")]
pub fn get_request_context() {}

/// Returns the default [`RequestOptions`] that should be used for a GraphQL request.
#[cfg(not(feature = "oss_release"))]
pub fn default_request_options() -> RequestOptions {
    RequestOptions {
        #[cfg(feature = "agent_mode_evals")]
        path_prefix: Some("/agent-mode-evals".to_string()),
        ..Default::default()
    }
}
