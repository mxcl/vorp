use std::collections::HashMap;

use crate::warp_managed_secrets::{
    client::{ManagedSecretValue, SecretOwner, TaskIdentityToken},
    ManagedSecret, ManagedSecretType,
};
#[cfg(not(feature = "oss_release"))]
use anyhow::Context;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
#[cfg(not(feature = "oss_release"))]
use cynic::{MutationBuilder, QueryBuilder};
#[cfg(not(feature = "oss_release"))]
use warp_graphql::mutations::issue_task_identity_token::{
    IssueTaskIdentityToken, IssueTaskIdentityTokenInput, IssueTaskIdentityTokenResult,
    IssueTaskIdentityTokenVariables,
};
#[cfg(not(feature = "oss_release"))]
use warp_graphql::object_permissions::OwnerType;
#[cfg(not(feature = "oss_release"))]
use warp_graphql::queries::list_managed_secrets::{
    ListManagedSecrets, ListManagedSecretsVariables, ManagedSecretsInput, ManagedSecretsResult,
};
#[cfg(not(feature = "oss_release"))]
use warp_graphql::queries::managed_secret_config::{
    GetManagedSecretConfig, GetManagedSecretConfigVariables, UserResult,
};
#[cfg(not(feature = "oss_release"))]
use warp_graphql::queries::task_secrets::{
    TaskSecrets, TaskSecretsInput, TaskSecretsResult, TaskSecretsVariables,
};
#[cfg(not(feature = "oss_release"))]
use warp_graphql::{
    mutations::{
        create_managed_secret::{
            CreateManagedSecret, CreateManagedSecretInput, CreateManagedSecretResult,
            CreateManagedSecretVariables,
        },
        delete_managed_secret::{
            DeleteManagedSecret, DeleteManagedSecretInput, DeleteManagedSecretResult,
            DeleteManagedSecretVariables,
        },
        update_managed_secret::{
            UpdateManagedSecret, UpdateManagedSecretInput, UpdateManagedSecretResult,
            UpdateManagedSecretVariables,
        },
    },
    object_permissions::Owner,
};

use super::ServerApi;
#[cfg(not(feature = "oss_release"))]
use crate::server::graphql::{get_request_context, get_user_facing_error_message};

pub use crate::warp_managed_secrets::client::{ManagedSecretConfigs, ManagedSecretsClient};

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
#[cfg(not(feature = "oss_release"))]
impl ManagedSecretsClient for ServerApi {
    async fn get_managed_secret_configs(&self) -> Result<ManagedSecretConfigs> {
        let variables = GetManagedSecretConfigVariables {
            request_context: get_request_context(),
        };
        let operation = GetManagedSecretConfig::build(variables);
        let response = self.send_graphql_request(operation, None).await?;

        match response.user {
            UserResult::UserOutput(output) => {
                let mut team_configs = HashMap::new();
                for workspace in output.user.workspaces {
                    for team in workspace.teams {
                        if let Some(config) = team.managed_secrets {
                            // DO NOT inline the `insert` call into the `debug_assert!` macro. It will get compiled out in release builds.
                            let prior_config =
                                team_configs.insert(team.uid.into_inner(), managed_secret_config(config));
                            debug_assert!(
                                prior_config.is_none(),
                                "Duplicate team UID returned from server"
                            );
                        }
                    }
                }
                Ok(ManagedSecretConfigs {
                    user_secrets: output.user.managed_secrets.map(managed_secret_config),
                    team_secrets: team_configs,
                })
            }
            UserResult::UserFacingError(error) => {
                Err(anyhow!(get_user_facing_error_message(error)))
            }
            UserResult::Unknown => Err(anyhow!(
                "Unknown error while getting managed secret configs"
            )),
        }
    }

    async fn create_managed_secret(
        &self,
        owner: SecretOwner,
        name: String,
        secret_type: ManagedSecretType,
        encrypted_value: String,
        description: Option<String>,
    ) -> Result<ManagedSecret> {
        let graphql_owner = match owner {
            SecretOwner::CurrentUser => Owner {
                type_: OwnerType::User,
                uid: None,
            },
            SecretOwner::Team { team_uid } => Owner {
                type_: OwnerType::Team,
                uid: Some(cynic::Id::new(team_uid)),
            },
        };

        let variables = CreateManagedSecretVariables {
            input: CreateManagedSecretInput {
                description,
                encrypted_value,
                name,
                owner: graphql_owner,
                type_: secret_type,
            },
            request_context: get_request_context(),
        };
        let operation = CreateManagedSecret::build(variables);
        let response = self.send_graphql_request(operation, None).await?;

        match response.create_managed_secret {
            CreateManagedSecretResult::CreateManagedSecretOutput(output) => {
                Ok(managed_secret(output.managed_secret))
            }
            CreateManagedSecretResult::UserFacingError(error) => {
                Err(anyhow!(get_user_facing_error_message(error)))
            }
            CreateManagedSecretResult::Unknown => {
                Err(anyhow!("Unknown error while creating managed secret"))
            }
        }
    }

    async fn delete_managed_secret(&self, owner: SecretOwner, name: String) -> Result<()> {
        let graphql_owner = match owner {
            SecretOwner::CurrentUser => Owner {
                type_: OwnerType::User,
                uid: None,
            },
            SecretOwner::Team { team_uid } => Owner {
                type_: OwnerType::Team,
                uid: Some(cynic::Id::new(team_uid)),
            },
        };

        let variables = DeleteManagedSecretVariables {
            input: DeleteManagedSecretInput {
                name,
                owner: graphql_owner,
            },
            request_context: get_request_context(),
        };
        let operation = DeleteManagedSecret::build(variables);
        let response = self.send_graphql_request(operation, None).await?;

        match response.delete_managed_secret {
            DeleteManagedSecretResult::DeleteManagedSecretOutput(_) => Ok(()),
            DeleteManagedSecretResult::UserFacingError(error) => {
                Err(anyhow!(get_user_facing_error_message(error)))
            }
            DeleteManagedSecretResult::Unknown => {
                Err(anyhow!("Unknown error while deleting managed secret"))
            }
        }
    }

    async fn update_managed_secret(
        &self,
        owner: SecretOwner,
        name: String,
        encrypted_value: Option<String>,
        description: Option<String>,
    ) -> Result<ManagedSecret> {
        let graphql_owner = match owner {
            SecretOwner::CurrentUser => Owner {
                type_: OwnerType::User,
                uid: None,
            },
            SecretOwner::Team { team_uid } => Owner {
                type_: OwnerType::Team,
                uid: Some(cynic::Id::new(team_uid)),
            },
        };

        let variables = UpdateManagedSecretVariables {
            input: UpdateManagedSecretInput {
                name,
                owner: graphql_owner,
                encrypted_value,
                description,
            },
            request_context: get_request_context(),
        };
        let operation = UpdateManagedSecret::build(variables);
        let response = self.send_graphql_request(operation, None).await?;

        match response.update_managed_secret {
            UpdateManagedSecretResult::UpdateManagedSecretOutput(output) => {
                Ok(managed_secret(output.managed_secret))
            }
            UpdateManagedSecretResult::UserFacingError(error) => {
                Err(anyhow!(get_user_facing_error_message(error)))
            }
            UpdateManagedSecretResult::Unknown => {
                Err(anyhow!("Unknown error while updating managed secret"))
            }
        }
    }

    async fn list_secrets(&self) -> Result<Vec<ManagedSecret>> {
        let variables = ListManagedSecretsVariables {
            // Pagination over managed secrets is not yet supported.
            input: ManagedSecretsInput { cursor: None },
            request_context: get_request_context(),
        };
        let operation = ListManagedSecrets::build(variables);
        let response = self.send_graphql_request(operation, None).await?;

        match response.managed_secrets {
            ManagedSecretsResult::ManagedSecretsOutput(output) => {
                Ok(output.managed_secrets.into_iter().map(managed_secret).collect())
            }
            ManagedSecretsResult::UserFacingError(error) => {
                Err(anyhow!(get_user_facing_error_message(error)))
            }
            ManagedSecretsResult::Unknown => {
                Err(anyhow!("Unknown error while listing managed secrets"))
            }
        }
    }

    async fn get_task_secrets(
        &self,
        task_id: String,
        workload_token: String,
    ) -> Result<HashMap<String, ManagedSecretValue>> {
        let variables = TaskSecretsVariables {
            input: TaskSecretsInput {
                task_id: cynic::Id::new(task_id),
                workload_token,
            },
            request_context: get_request_context(),
        };
        let operation = TaskSecrets::build(variables);
        let response = self.send_graphql_request(operation, None).await?;

        match response.task_secrets {
            TaskSecretsResult::TaskSecretsOutput(output) => {
                let mut secrets = HashMap::new();
                for entry in output.secrets {
                    secrets.insert(entry.name, managed_secret_value(entry.value)?);
                }
                Ok(secrets)
            }
            TaskSecretsResult::UserFacingError(error) => {
                Err(anyhow!(get_user_facing_error_message(error)))
            }
            TaskSecretsResult::Unknown => Err(anyhow!("Unknown error while getting task secrets")),
        }
    }

    async fn issue_task_identity_token(
        &self,
        options: crate::warp_managed_secrets::client::IdentityTokenOptions,
    ) -> Result<TaskIdentityToken> {
        let requested_duration_seconds = options
            .requested_duration
            .as_secs()
            .try_into()
            .context("Requested duration out of bounds")?;
        let variables = IssueTaskIdentityTokenVariables {
            input: IssueTaskIdentityTokenInput {
                audience: options.audience,
                requested_duration_seconds,
                subject_template: Some(options.subject_template.into_vec()),
            },
            request_context: get_request_context(),
        };
        let operation = IssueTaskIdentityToken::build(variables);
        let response = self.send_graphql_request(operation, None).await?;

        match response.issue_task_identity_token {
            IssueTaskIdentityTokenResult::IssueTaskIdentityTokenOutput(output) => {
                Ok(TaskIdentityToken {
                    token: output.token,
                    expires_at: output.expires_at.utc(),
                    issuer: output.issuer,
                })
            }
            IssueTaskIdentityTokenResult::UserFacingError(error) => {
                Err(anyhow!(get_user_facing_error_message(error)))
            }
            IssueTaskIdentityTokenResult::Unknown => {
                Err(anyhow!("Unknown error while issuing task identity token"))
            }
        }
    }
}

#[cfg(all(not(feature = "oss_release"), feature = "warp_managed_secrets"))]
fn managed_secret_config(
    config: warp_graphql::managed_secrets::ManagedSecretConfig,
) -> crate::warp_managed_secrets::ManagedSecretConfig {
    config
}

#[cfg(all(not(feature = "oss_release"), not(feature = "warp_managed_secrets")))]
fn managed_secret_config(
    config: warp_graphql::managed_secrets::ManagedSecretConfig,
) -> crate::warp_managed_secrets::ManagedSecretConfig {
    crate::warp_managed_secrets::ManagedSecretConfig {
        public_key: config.public_key,
    }
}

#[cfg(all(not(feature = "oss_release"), feature = "warp_managed_secrets"))]
fn managed_secret(secret: warp_graphql::managed_secrets::ManagedSecret) -> ManagedSecret {
    secret
}

#[cfg(all(not(feature = "oss_release"), not(feature = "warp_managed_secrets")))]
fn managed_secret(secret: warp_graphql::managed_secrets::ManagedSecret) -> ManagedSecret {
    ManagedSecret {
        name: secret.name,
        description: secret.description,
        created_at: server_timestamp(secret.created_at),
        updated_at: server_timestamp(secret.updated_at),
        owner: crate::warp_managed_secrets::ManagedSecretOwner {
            uid: secret.owner.uid.into_inner(),
            type_: match secret.owner.type_ {
                warp_graphql::object::SpaceType::Team => {
                    crate::warp_managed_secrets::ManagedSecretOwnerType::Team
                }
                warp_graphql::object::SpaceType::User => {
                    crate::warp_managed_secrets::ManagedSecretOwnerType::User
                }
            },
        },
        type_: managed_secret_type(secret.type_),
    }
}

#[cfg(all(not(feature = "oss_release"), not(feature = "warp_managed_secrets")))]
fn server_timestamp(
    timestamp: warp_graphql::scalars::time::ServerTimestamp,
) -> crate::server::timestamp::ServerTimestamp {
    crate::server::timestamp::ServerTimestamp::from_unix_timestamp_micros(
        timestamp.timestamp_micros(),
    )
    .expect("GraphQL timestamp should round-trip through microseconds")
}

#[cfg(all(not(feature = "oss_release"), not(feature = "warp_managed_secrets")))]
fn managed_secret_type(
    type_: warp_graphql::managed_secrets::ManagedSecretType,
) -> ManagedSecretType {
    match type_ {
        warp_graphql::managed_secrets::ManagedSecretType::AnthropicApiKey => {
            ManagedSecretType::AnthropicApiKey
        }
        warp_graphql::managed_secrets::ManagedSecretType::AnthropicBedrockAccessKey => {
            ManagedSecretType::AnthropicBedrockAccessKey
        }
        warp_graphql::managed_secrets::ManagedSecretType::AnthropicBedrockApiKey => {
            ManagedSecretType::AnthropicBedrockApiKey
        }
        warp_graphql::managed_secrets::ManagedSecretType::Dotenvx => ManagedSecretType::Dotenvx,
        warp_graphql::managed_secrets::ManagedSecretType::RawValue => ManagedSecretType::RawValue,
    }
}

#[cfg(all(not(feature = "oss_release"), feature = "warp_managed_secrets"))]
fn managed_secret_value(
    value: warp_graphql::queries::task_secrets::ManagedSecretValue,
) -> Result<ManagedSecretValue> {
    Ok(value)
}

#[cfg(all(not(feature = "oss_release"), not(feature = "warp_managed_secrets")))]
fn managed_secret_value(
    value: warp_graphql::queries::task_secrets::ManagedSecretValue,
) -> Result<ManagedSecretValue> {
    match value {
        warp_graphql::queries::task_secrets::ManagedSecretValue::ManagedSecretRawValue(raw) => {
            Ok(ManagedSecretValue::raw_value(raw.value))
        }
        warp_graphql::queries::task_secrets::ManagedSecretValue::ManagedSecretAnthropicApiKeyValue(
            value,
        ) => Ok(ManagedSecretValue::anthropic_api_key(value.api_key)),
        warp_graphql::queries::task_secrets::ManagedSecretValue::ManagedSecretAnthropicBedrockAccessKeyValue(
            value,
        ) => Ok(ManagedSecretValue::anthropic_bedrock_access_key(
            value.aws_access_key_id,
            value.aws_secret_access_key,
            value.aws_session_token,
            value.aws_region,
        )),
        warp_graphql::queries::task_secrets::ManagedSecretValue::ManagedSecretAnthropicBedrockApiKeyValue(
            value,
        ) => Ok(ManagedSecretValue::anthropic_bedrock_api_key(
            value.aws_bearer_token_bedrock,
            value.aws_region,
        )),
        warp_graphql::queries::task_secrets::ManagedSecretValue::Unknown => {
            Err(anyhow!("Unknown managed secret value type"))
        }
    }
}

#[cfg(feature = "oss_release")]
fn managed_secrets_unavailable<T>() -> Result<T> {
    Err(anyhow!(
        "Managed secrets server APIs are not available in this build"
    ))
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
#[cfg(feature = "oss_release")]
impl ManagedSecretsClient for ServerApi {
    async fn get_managed_secret_configs(&self) -> Result<ManagedSecretConfigs> {
        Ok(ManagedSecretConfigs {
            user_secrets: None,
            team_secrets: HashMap::new(),
        })
    }

    async fn create_managed_secret(
        &self,
        _owner: SecretOwner,
        _name: String,
        _secret_type: ManagedSecretType,
        _encrypted_value: String,
        _description: Option<String>,
    ) -> Result<ManagedSecret> {
        managed_secrets_unavailable()
    }

    async fn delete_managed_secret(&self, _owner: SecretOwner, _name: String) -> Result<()> {
        Ok(())
    }

    async fn update_managed_secret(
        &self,
        _owner: SecretOwner,
        _name: String,
        _encrypted_value: Option<String>,
        _description: Option<String>,
    ) -> Result<ManagedSecret> {
        managed_secrets_unavailable()
    }

    async fn list_secrets(&self) -> Result<Vec<ManagedSecret>> {
        Ok(Vec::new())
    }

    async fn get_task_secrets(
        &self,
        _task_id: String,
        _workload_token: String,
    ) -> Result<HashMap<String, ManagedSecretValue>> {
        Ok(HashMap::new())
    }

    async fn issue_task_identity_token(
        &self,
        _options: crate::warp_managed_secrets::client::IdentityTokenOptions,
    ) -> Result<TaskIdentityToken> {
        managed_secrets_unavailable()
    }
}
