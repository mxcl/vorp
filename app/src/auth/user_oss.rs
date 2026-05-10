use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

use crate::server::timestamp::ServerTimestamp;

use super::UserUid;

pub use warp_server_client::auth::{TEST_USER_EMAIL, TEST_USER_UID};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnonymousUserType {
    NativeClientAnonymousUser,
    NativeClientAnonymousUserFeatureGated,
    WebClientAnonymousUser,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PrincipalType {
    #[default]
    User,
    ServiceAccount,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct PersonalObjectLimits {
    pub env_var_limit: usize,
    pub notebook_limit: usize,
    pub workflow_limit: usize,
}

#[derive(Debug, Clone)]
pub struct User {
    pub local_id: UserUid,
    pub metadata: UserMetadata,
    pub is_onboarded: bool,
    pub needs_sso_link: bool,
    pub anonymous_user_type: Option<AnonymousUserType>,
    pub is_on_work_domain: bool,
    pub linked_at: Option<ServerTimestamp>,
    pub personal_object_limits: Option<PersonalObjectLimits>,
    pub principal_type: PrincipalType,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UserMetadata {
    pub email: String,
    pub display_name: Option<String>,
    pub photo_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FirebaseAuthTokens {
    pub id_token: String,
    pub refresh_token: String,
    pub expiration_time: DateTime<FixedOffset>,
}

impl FirebaseAuthTokens {
    pub fn from_response(
        id_token: String,
        refresh_token: String,
        expires_in: String,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            id_token,
            expiration_time: Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())
                + chrono::Duration::seconds(
                    expires_in.parse::<i64>().map_err(anyhow::Error::from)?,
                ),
            refresh_token,
        })
    }
}

impl User {
    pub fn username_for_display(&self) -> &str {
        self.metadata
            .display_name
            .as_deref()
            .unwrap_or(self.metadata.email.as_str())
    }

    pub fn display_name(&self) -> Option<String> {
        self.metadata.display_name.clone()
    }

    pub fn test() -> Self {
        Self {
            local_id: UserUid::new(TEST_USER_UID),
            metadata: UserMetadata {
                email: TEST_USER_EMAIL.to_owned(),
                display_name: None,
                photo_url: None,
            },
            is_onboarded: true,
            needs_sso_link: false,
            anonymous_user_type: None,
            is_on_work_domain: false,
            linked_at: None,
            personal_object_limits: None,
            principal_type: PrincipalType::User,
        }
    }

    pub fn is_user_anonymous(&self) -> bool {
        self.anonymous_user_type().is_some() && self.linked_at().is_none()
    }

    pub fn anonymous_user_type(&self) -> Option<AnonymousUserType> {
        self.anonymous_user_type
    }

    pub fn personal_object_limits(&self) -> Option<PersonalObjectLimits> {
        self.personal_object_limits
    }

    pub fn linked_at(&self) -> Option<ServerTimestamp> {
        self.linked_at
    }
}
