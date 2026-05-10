use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirebaseError {
    pub code: i32,
    pub message: String,
}

impl std::error::Error for FirebaseError {}

impl std::fmt::Display for FirebaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Authentication request failed")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub local_id: String,
    pub photo_url: Option<String>,
    pub screen_name: Option<String>,
}

impl AccountInfo {
    pub fn from_profile(
        firebase_uid: String,
        photo_url: Option<String>,
        display_name: Option<String>,
        _email: Option<String>,
    ) -> Self {
        Self {
            local_id: firebase_uid,
            photo_url,
            screen_name: display_name,
        }
    }

    pub fn display_name(&self) -> Option<&str> {
        self.screen_name.as_deref()
    }

    pub fn email(&self) -> anyhow::Result<&str> {
        anyhow::bail!("Authentication is not available in this build")
    }

    pub fn has_sso_link(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountInfoResponsePayload {
    users: Vec<AccountInfo>,
}

impl GetAccountInfoResponsePayload {
    pub fn user_account_info(self) -> anyhow::Result<AccountInfo> {
        self.users
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("Authentication is not available in this build"))
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
