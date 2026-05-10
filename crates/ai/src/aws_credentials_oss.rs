use std::time::SystemTime;

use warp_core::ui::Icon;

/// Placeholder credentials for the OSS build. AI provider credentials are not
/// loaded or persisted in this build.
#[derive(Clone, PartialEq, Eq)]
pub struct AwsCredentials {
    expires_at: Option<SystemTime>,
}

impl AwsCredentials {
    pub fn new(
        _access_key: String,
        _secret_key: String,
        _session_token: Option<String>,
        expires_at: Option<SystemTime>,
    ) -> Self {
        Self { expires_at }
    }

    pub fn expires_at(&self) -> Option<SystemTime> {
        self.expires_at
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum AwsCredentialsState {
    Missing,
    Disabled,
    Refreshing,
    Loaded {
        credentials: AwsCredentials,
        loaded_at: SystemTime,
    },
    Failed {
        message: String,
    },
}

impl AwsCredentialsState {
    pub fn user_facing_components(&self) -> (String, String, Icon) {
        (
            "Unavailable".to_string(),
            "AI provider credentials are not available in this build".to_string(),
            Icon::Key,
        )
    }
}
