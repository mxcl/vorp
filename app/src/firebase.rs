use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirebaseError {
    pub code: i32,
    pub message: String,
}

impl std::error::Error for FirebaseError {}

impl std::fmt::Display for FirebaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Firebase request failed with status {} and message: {}",
            self.code, self.message
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProviderUserInfo {
    display_name: Option<String>,
    email: Option<String>,
    provider_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub local_id: String,
    pub photo_url: Option<String>,
    pub screen_name: Option<String>,
    display_name: Option<String>,
    email: Option<String>,
    #[serde(default)]
    provider_user_info: Vec<ProviderUserInfo>,
}

impl AccountInfo {
    pub fn from_profile(
        firebase_uid: String,
        photo_url: Option<String>,
        display_name: Option<String>,
        email: Option<String>,
    ) -> Self {
        Self {
            local_id: firebase_uid,
            photo_url,
            screen_name: display_name.clone(),
            display_name,
            email,
            provider_user_info: vec![],
        }
    }

    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref().or_else(|| {
            self.provider_user_info
                .iter()
                .find_map(|user_info| user_info.display_name.as_deref())
        })
    }

    pub fn email(&self) -> Result<&str> {
        self.email
            .as_deref()
            .or_else(|| {
                self.provider_user_info
                    .iter()
                    .find_map(|user_info| user_info.email.as_deref())
            })
            .ok_or_else(|| anyhow!("Email address missing from user information"))
    }

    pub fn has_sso_link(&self) -> bool {
        self.provider_user_info
            .iter()
            .any(|user_info| user_info.provider_id.as_deref() == Some("oidc.workos"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountInfoResponsePayload {
    users: Vec<AccountInfo>,
}

impl GetAccountInfoResponsePayload {
    pub fn user_account_info(self) -> Result<AccountInfo> {
        self.users.into_iter().next().ok_or_else(|| {
            anyhow!("field `users` was unexpectedly empty in GetAccountInfoResponse")
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountInfoResponse {
    Success(GetAccountInfoResponsePayload),
    Error { error: FirebaseError },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchAccessTokenResponse {
    Success {
        #[serde(alias = "expiresIn")]
        expires_in: String,
        #[serde(alias = "idToken")]
        id_token: String,
        #[serde(alias = "refreshToken")]
        refresh_token: String,
    },
    Error {
        error: FirebaseError,
    },
}
