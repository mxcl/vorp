use std::{
    collections::HashMap,
    ffi::OsString,
    future::Future,
    time::{Duration, SystemTime},
};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use vec1::Vec1;
use warp_graphql::managed_secrets::{ManagedSecret, ManagedSecretConfig, ManagedSecretType};
use warpui::{Entity, SingletonEntity};

pub mod client {
    use super::*;

    pub use warp_graphql::queries::task_secrets::ManagedSecretValue as GraphQLManagedSecretValue;

    #[derive(Debug, Clone)]
    pub struct TaskIdentityToken {
        pub token: String,
        pub expires_at: DateTime<Utc>,
        pub issuer: String,
    }

    pub struct IdentityTokenOptions {
        pub audience: String,
        pub requested_duration: Duration,
        pub subject_template: Vec1<String>,
    }

    #[derive(Debug)]
    pub struct ManagedSecretConfigs {
        pub user_secrets: Option<ManagedSecretConfig>,
        pub team_secrets: HashMap<String, ManagedSecretConfig>,
    }

    #[derive(Debug, Clone)]
    pub enum SecretOwner {
        CurrentUser,
        Team { team_uid: String },
    }

    #[cfg_attr(not(target_family = "wasm"), async_trait)]
    #[cfg_attr(target_family = "wasm", async_trait(?Send))]
    pub trait ManagedSecretsClient: 'static + Send + Sync {
        async fn get_managed_secret_configs(&self) -> Result<ManagedSecretConfigs>;

        async fn create_managed_secret(
            &self,
            owner: SecretOwner,
            name: String,
            secret_type: ManagedSecretType,
            encrypted_value: String,
            description: Option<String>,
        ) -> Result<ManagedSecret>;

        async fn delete_managed_secret(&self, owner: SecretOwner, name: String) -> Result<()>;

        async fn update_managed_secret(
            &self,
            owner: SecretOwner,
            name: String,
            encrypted_value: Option<String>,
            description: Option<String>,
        ) -> Result<ManagedSecret>;

        async fn list_secrets(&self) -> Result<Vec<ManagedSecret>>;

        async fn get_task_secrets(
            &self,
            task_id: String,
            workload_token: String,
        ) -> Result<HashMap<String, GraphQLManagedSecretValue>>;

        async fn issue_task_identity_token(
            &self,
            options: IdentityTokenOptions,
        ) -> Result<TaskIdentityToken>;
    }
}

#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum ManagedSecretValue {
    RawValue {
        value: String,
    },
    AnthropicApiKey {
        api_key: String,
    },
    AnthropicBedrockAccessKey {
        aws_access_key_id: String,
        aws_secret_access_key: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        aws_session_token: Option<String>,
        aws_region: String,
    },
    AnthropicBedrockApiKey {
        aws_bearer_token_bedrock: String,
        aws_region: String,
    },
}

impl ManagedSecretValue {
    pub fn raw_value(s: impl Into<String>) -> Self {
        Self::RawValue { value: s.into() }
    }

    pub fn anthropic_api_key(s: impl Into<String>) -> Self {
        Self::AnthropicApiKey { api_key: s.into() }
    }

    pub fn anthropic_bedrock_access_key(
        access_key_id: impl Into<String>,
        secret_access_key: impl Into<String>,
        session_token: Option<String>,
        region: impl Into<String>,
    ) -> Self {
        Self::AnthropicBedrockAccessKey {
            aws_access_key_id: access_key_id.into(),
            aws_secret_access_key: secret_access_key.into(),
            aws_session_token: session_token,
            aws_region: region.into(),
        }
    }

    pub fn anthropic_bedrock_api_key(token: impl Into<String>, region: impl Into<String>) -> Self {
        Self::AnthropicBedrockApiKey {
            aws_bearer_token_bedrock: token.into(),
            aws_region: region.into(),
        }
    }

    pub fn secret_type(&self) -> ManagedSecretType {
        match self {
            ManagedSecretValue::RawValue { .. } => ManagedSecretType::RawValue,
            ManagedSecretValue::AnthropicApiKey { .. } => ManagedSecretType::AnthropicApiKey,
            ManagedSecretValue::AnthropicBedrockAccessKey { .. } => {
                ManagedSecretType::AnthropicBedrockAccessKey
            }
            ManagedSecretValue::AnthropicBedrockApiKey { .. } => {
                ManagedSecretType::AnthropicBedrockApiKey
            }
        }
    }
}

impl std::fmt::Debug for ManagedSecretValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManagedSecretValue::RawValue { .. } => f
                .debug_struct("ManagedSecret::RawValue")
                .finish_non_exhaustive(),
            ManagedSecretValue::AnthropicApiKey { .. } => f
                .debug_struct("ManagedSecret::AnthropicApiKey")
                .finish_non_exhaustive(),
            ManagedSecretValue::AnthropicBedrockAccessKey { .. } => f
                .debug_struct("ManagedSecret::AnthropicBedrockAccessKey")
                .finish_non_exhaustive(),
            ManagedSecretValue::AnthropicBedrockApiKey { .. } => f
                .debug_struct("ManagedSecret::AnthropicBedrockApiKey")
                .finish_non_exhaustive(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GcpFederationConfig {
    pub project_number: String,
    pub pool_id: String,
    pub provider_id: String,
    pub service_account_email: Option<String>,
    pub token_lifetime: Option<Duration>,
}

pub struct GcpCredentials;

impl GcpCredentials {
    pub fn federated(
        _run_id: &str,
        _config: &GcpFederationConfig,
    ) -> std::result::Result<Self, PrepareGcpCredentialsError> {
        Err(PrepareGcpCredentialsError::Unavailable)
    }

    pub fn env_vars(&self) -> HashMap<OsString, OsString> {
        HashMap::new()
    }

    pub fn cleanup(self) -> std::result::Result<(), PrepareGcpCredentialsError> {
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PrepareGcpCredentialsError {
    #[error("managed secrets are not available in this build")]
    Unavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcpWorkloadIdentityFederationToken {
    pub version: i32,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, thiserror::Error, Serialize)]
pub enum GcpWorkloadIdentityFederationError {
    #[error("managed secrets are not available in this build")]
    Unavailable,
}

pub trait ActorProvider: Send + Sync + 'static {
    fn actor_uid(&self) -> Option<String>;
}

pub struct ManagedSecretManager;

impl ManagedSecretManager {
    pub fn new(
        _client: std::sync::Arc<dyn client::ManagedSecretsClient>,
        _actor_provider: std::sync::Arc<dyn ActorProvider>,
    ) -> Self {
        Self
    }

    pub fn create_secret(
        &self,
        _owner: client::SecretOwner,
        _name: String,
        _value: ManagedSecretValue,
        _description: Option<String>,
    ) -> impl Future<Output = Result<ManagedSecret>> + use<> {
        async { Err(unavailable()) }
    }

    pub fn delete_secret(
        &self,
        _owner: client::SecretOwner,
        _name: String,
    ) -> impl Future<Output = Result<()>> + use<> {
        async { Err(unavailable()) }
    }

    pub fn update_secret(
        &self,
        _owner: client::SecretOwner,
        _name: String,
        _value: Option<ManagedSecretValue>,
        _description: Option<String>,
    ) -> impl Future<Output = Result<ManagedSecret>> + use<> {
        async { Err(unavailable()) }
    }

    pub fn list_secrets(&self) -> impl Future<Output = Result<Vec<ManagedSecret>>> + use<> {
        async { Ok(Vec::new()) }
    }

    pub fn get_task_secrets(
        &self,
        _task_id: String,
    ) -> impl Future<Output = Result<HashMap<String, ManagedSecretValue>>> + use<> {
        async { Ok(HashMap::new()) }
    }

    pub fn issue_task_identity_token(
        &self,
        _options: client::IdentityTokenOptions,
    ) -> impl Future<Output = Result<client::TaskIdentityToken>> + use<> {
        async { Err(unavailable()) }
    }

    pub fn issue_gcp_workload_identity_federation_token(
        &self,
        _audience: String,
        _token_type: String,
        _duration: Duration,
    ) -> impl Future<
        Output = std::result::Result<
            GcpWorkloadIdentityFederationToken,
            GcpWorkloadIdentityFederationError,
        >,
    > + use<> {
        async { Err(GcpWorkloadIdentityFederationError::Unavailable) }
    }
}

impl Entity for ManagedSecretManager {
    type Event = ();
}

impl SingletonEntity for ManagedSecretManager {}

fn unavailable() -> anyhow::Error {
    anyhow!("managed secrets are not available in this build")
}

#[allow(dead_code)]
fn unix_expiration(seconds_from_now: u64) -> u64 {
    SystemTime::now()
        .checked_add(Duration::from_secs(seconds_from_now))
        .and_then(|time| time.duration_since(SystemTime::UNIX_EPOCH).ok())
        .map(|duration| duration.as_secs())
        .unwrap_or(seconds_from_now)
}
