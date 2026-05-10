use super::ServerApi;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
#[cfg(not(feature = "oss_release"))]
use cynic::{MutationBuilder, QueryBuilder};

#[cfg(not(feature = "oss_release"))]
use crate::channel::ChannelState;
#[cfg(not(feature = "oss_release"))]
use crate::features::FeatureFlag;
#[cfg(test)]
use mockall::automock;

#[cfg(not(feature = "oss_release"))]
use crate::server::graphql::{get_request_context, get_user_facing_error_message};
#[cfg(not(feature = "oss_release"))]
use warp_graphql::mutations::create_simple_integration::{
    CreateSimpleIntegration, CreateSimpleIntegrationResult, CreateSimpleIntegrationVariables,
    SimpleIntegrationConfig,
};
#[cfg(not(feature = "oss_release"))]
use warp_graphql::queries::get_integrations_using_environment::{
    GetIntegrationsUsingEnvironment, GetIntegrationsUsingEnvironmentInput,
    GetIntegrationsUsingEnvironmentResult, GetIntegrationsUsingEnvironmentVariables,
};
#[cfg(not(feature = "oss_release"))]
use warp_graphql::queries::get_oauth_connect_tx_status::{
    GetOAuthConnectTxStatus, GetOAuthConnectTxStatusInput, GetOAuthConnectTxStatusResult,
    GetOAuthConnectTxStatusVariables,
};
#[cfg(not(feature = "oss_release"))]
use warp_graphql::queries::get_simple_integrations::{
    SimpleIntegrations, SimpleIntegrationsInput, SimpleIntegrationsResult,
    SimpleIntegrationsVariables,
};
#[cfg(not(feature = "oss_release"))]
use warp_graphql::queries::suggest_cloud_environment_image::{
    RepoInput as SuggestCloudEnvironmentImageRepoInput, SuggestCloudEnvironmentImage,
    SuggestCloudEnvironmentImageInput, SuggestCloudEnvironmentImageVariables,
};
#[cfg(not(feature = "oss_release"))]
use warp_graphql::queries::user_github_info::{
    UserGithubInfo, UserGithubInfoVariables,
};
#[cfg(not(feature = "oss_release"))]
use warp_graphql::queries::user_repo_auth_status::{
    RepoInput as UserRepoAuthStatusRepoInput, UserRepoAuthStatus as GqlUserRepoAuthStatus,
    UserRepoAuthStatusInput, UserRepoAuthStatusResult, UserRepoAuthStatusVariables,
};
use crate::server::timestamp::ServerTimestamp;

#[derive(Debug)]
pub struct UserRepoAuthStatusOutput {
    pub statuses: Vec<UserRepoAuthStatus>,
    pub auth_url: Option<String>,
    pub tx_id: Option<String>,
}

#[derive(Debug)]
pub struct UserRepoAuthStatus {
    pub owner: String,
    pub repo: String,
    pub status: UserRepoAuthStatusEnum,
    pub is_public: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum UserRepoAuthStatusEnum {
    NoInstallationOrAccessForRepo,
    UserNotConnectedToGithub,
    Success,
}

pub struct CreateSimpleIntegrationOutput {
    pub auth_url: Option<String>,
    pub success: bool,
    pub message: String,
    pub tx_id: Option<String>,
}

#[derive(Clone, Debug)]
pub struct SimpleIntegrationsOutput {
    pub integrations: Vec<SimpleIntegration>,
    pub message: Option<String>,
}

#[derive(Clone, Debug)]
pub struct SimpleIntegration {
    pub provider_slug: String,
    pub description: String,
    pub connection_status: SimpleIntegrationConnectionStatus,
    pub integration_config: Option<ListedSimpleIntegrationConfig>,
    pub created_at: Option<ServerTimestamp>,
    pub updated_at: Option<ServerTimestamp>,
}

#[derive(Clone, Debug)]
pub struct ListedSimpleIntegrationConfig {
    pub environment_uid: String,
    pub base_prompt: String,
    pub model_id: String,
    pub mcp_servers_json: String,
}

#[derive(Clone, Copy, Debug)]
pub enum SimpleIntegrationConnectionStatus {
    NotConnected,
    ConnectionError,
    IntegrationNotConfigured,
    NotEnabled,
    Active,
}

#[derive(Clone, Copy, Debug)]
pub enum OauthConnectTxStatus {
    Completed,
    Expired,
    Failed,
    InProgress,
    Pending,
}

#[derive(Clone, Debug)]
pub struct GetIntegrationsUsingEnvironmentOutput {
    pub provider_names: Vec<String>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct GithubConnectedOutput {
    pub username: Option<String>,
    pub installed_repos: Vec<GithubRepoResult>,
    pub app_install_link: String,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct GithubAuthRequiredOutput {
    pub auth_url: String,
    pub tx_id: String,
    pub app_install_link: String,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct GithubRepoResult {
    pub owner: String,
    pub repo: String,
    pub is_public: bool,
}

#[derive(Clone, Debug)]
pub enum UserGithubInfoResult {
    GithubConnectedOutput(GithubConnectedOutput),
    GithubAuthRequiredOutput(GithubAuthRequiredOutput),
    Unknown,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SuggestCloudEnvironmentImageAuthRequiredOutput {
    pub auth_url: String,
    pub tx_id: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SuggestCloudEnvironmentImageOutput {
    pub detected_languages: Vec<GithubReposLanguageStat>,
    pub image: String,
    pub needs_custom_image: bool,
    pub reason: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct GithubReposLanguageStat {
    pub bytes: i32,
    pub language: String,
    pub percentage: f64,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum SuggestCloudEnvironmentImageResult {
    SuggestCloudEnvironmentImageAuthRequiredOutput(SuggestCloudEnvironmentImageAuthRequiredOutput),
    SuggestCloudEnvironmentImageOutput(SuggestCloudEnvironmentImageOutput),
    UserFacingError,
    Unknown,
}

#[cfg(not(target_family = "wasm"))]
pub trait IntegrationsClientBounds: Send + Sync {}

#[cfg(not(target_family = "wasm"))]
impl<T: 'static + Send + Sync> IntegrationsClientBounds for T {}

#[cfg(target_family = "wasm")]
pub trait IntegrationsClientBounds {}

#[cfg(target_family = "wasm")]
impl<T: 'static> IntegrationsClientBounds for T {}

#[cfg_attr(test, automock)]
#[cfg_attr(target_family = "wasm", allow(dead_code))]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
pub trait IntegrationsClient: 'static + IntegrationsClientBounds {
    /// Checks the user's GitHub authorization status for the given repositories.
    ///
    /// Returns a list of statuses for each repo, indicating whether the user has
    /// access to the repo, and an optional auth URL for the user to authorize.
    async fn check_user_repo_auth_status(
        &self,
        repos: Vec<(String, String)>,
    ) -> Result<UserRepoAuthStatusOutput>;

    /// Creates or updates a simple integration on the server.
    ///
    /// # Arguments
    /// * `integration_type` - The type of integration (e.g. "github", "linear", "slack")
    /// * `is_update` - Whether this is an update to an existing integration
    /// * `environment_uid` - The UID of the environment to associate with this integration
    /// * `base_prompt` - Optional base prompt for the integration
    /// * `model_id` - Optional model ID for the integration
    /// * `mcp_servers_json` - Optional JSON string encoding a map[string]MCPServerConfig (ambient agent spec)
    /// * `remove_mcp_server_names` - Optional list of MCP server names to remove (applies on update)
    /// * `worker_host` - Optional worker host ID for self-hosted workers
    /// * `enabled` - Whether the integration should be enabled on creation
    #[allow(clippy::too_many_arguments)]
    async fn create_or_update_simple_integration(
        &self,
        integration_type: String,
        is_update: bool,
        environment_uid: Option<String>,
        base_prompt: Option<String>,
        model_id: Option<String>,
        mcp_servers_json: Option<String>,
        remove_mcp_server_names: Option<Vec<String>>,
        worker_host: Option<String>,
        enabled: bool,
    ) -> Result<CreateSimpleIntegrationOutput>;

    /// Lists simple integrations for a fixed set of provider slugs.
    ///
    /// The server will return one SimpleIntegration entry per requested provider,
    /// regardless of whether the connection or integration currently exists.
    async fn list_simple_integrations(
        &self,
        providers: Vec<String>,
    ) -> Result<SimpleIntegrationsOutput>;

    /// Polls the status of an OAuth connect transaction.
    ///
    /// # Arguments
    /// * `tx_id` - The transaction ID returned from create_simple_integration
    ///
    /// # Returns
    /// * `Ok(OauthConnectTxStatus)` - The current status of the transaction
    /// * `Err` - If the transaction is not found or polling fails
    async fn poll_oauth_connect_status(&self, tx_id: String) -> Result<OauthConnectTxStatus>;

    /// Gets the list of integration provider names that are using the specified environment.
    ///
    /// # Arguments
    /// * `environment_id` - The ID of the environment to check
    ///
    /// # Returns
    /// * `Ok(Vec<String>)` - List of provider names (e.g., ["linear", "slack"]) using this environment
    /// * `Err` - If the query fails
    async fn get_integrations_using_environment(
        &self,
        environment_id: String,
    ) -> Result<GetIntegrationsUsingEnvironmentOutput>;

    /// Gets the user's GitHub connection info, including accessible repos.
    ///
    /// # Returns
    /// * `Ok(UserGithubInfoResult)` - Either connected with repos, or auth required
    /// * `Err` - If the query fails
    async fn get_user_github_info(&self) -> Result<UserGithubInfoResult>;

    /// Suggests a Docker image for a cloud environment based on the provided repos.
    async fn suggest_cloud_environment_image(
        &self,
        repos: Vec<(String, String)>,
    ) -> Result<SuggestCloudEnvironmentImageResult>;
}

#[cfg_attr(target_family = "wasm", async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg(not(feature = "oss_release"))]
impl IntegrationsClient for ServerApi {
    async fn check_user_repo_auth_status(
        &self,
        repos: Vec<(String, String)>,
    ) -> Result<UserRepoAuthStatusOutput> {
        let repo_inputs: Vec<UserRepoAuthStatusRepoInput> = repos
            .into_iter()
            .map(|(owner, repo)| UserRepoAuthStatusRepoInput { owner, repo })
            .collect();

        let variables = UserRepoAuthStatusVariables {
            request_context: get_request_context(),
            input: UserRepoAuthStatusInput { repos: repo_inputs },
        };

        let operation = GqlUserRepoAuthStatus::build(variables);
        let response = self.send_graphql_request(operation, None).await?;

        match response.user_repo_auth_status {
            UserRepoAuthStatusResult::UserRepoAuthStatusOutput(output) => {
                Ok(user_repo_auth_status_output(output))
            }
            UserRepoAuthStatusResult::Unknown => Err(anyhow::anyhow!(
                "Failed to check GitHub auth status: unknown response"
            )),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn create_or_update_simple_integration(
        &self,
        integration_type: String,
        is_update: bool,
        environment_uid: Option<String>,
        base_prompt: Option<String>,
        model_id: Option<String>,
        mcp_servers_json: Option<String>,
        remove_mcp_server_names: Option<Vec<String>>,
        worker_host: Option<String>,
        enabled: bool,
    ) -> Result<CreateSimpleIntegrationOutput> {
        let variables = CreateSimpleIntegrationVariables {
            config: SimpleIntegrationConfig {
                base_prompt,
                environment_uid,
                model_id,
                mcp_servers_json,
                remove_mcp_server_names,
                worker_host,
            },
            enabled,
            integration_type,
            is_update,
            request_context: get_request_context(),
        };

        let operation = CreateSimpleIntegration::build(variables);
        let response = self.send_graphql_request(operation, None).await?;
        match response.create_simple_integration {
            CreateSimpleIntegrationResult::CreateSimpleIntegrationOutput(output) => {
                Ok(create_simple_integration_output(output))
            }
            CreateSimpleIntegrationResult::UserFacingError(error) => {
                Err(anyhow!(get_user_facing_error_message(error)))
            }
            CreateSimpleIntegrationResult::Unknown => {
                Err(anyhow!("Unknown error while creating integration"))
            }
        }
    }

    async fn get_integrations_using_environment(
        &self,
        environment_id: String,
    ) -> Result<GetIntegrationsUsingEnvironmentOutput> {
        let variables = GetIntegrationsUsingEnvironmentVariables {
            request_context: get_request_context(),
            input: GetIntegrationsUsingEnvironmentInput { environment_id },
        };

        let operation = GetIntegrationsUsingEnvironment::build(variables);
        let response = self.send_graphql_request(operation, None).await?;

        match response.get_integrations_using_environment {
            GetIntegrationsUsingEnvironmentResult::GetIntegrationsUsingEnvironmentOutput(
                output,
            ) => Ok(GetIntegrationsUsingEnvironmentOutput {
                provider_names: output.provider_names,
            }),
            GetIntegrationsUsingEnvironmentResult::UserFacingError(error) => {
                Err(anyhow!(get_user_facing_error_message(error)))
            }
            GetIntegrationsUsingEnvironmentResult::Unknown => Err(anyhow!(
                "Unknown error while getting integrations using environment"
            )),
        }
    }

    async fn list_simple_integrations(
        &self,
        providers: Vec<String>,
    ) -> Result<SimpleIntegrationsOutput> {
        let variables = SimpleIntegrationsVariables {
            request_context: get_request_context(),
            input: SimpleIntegrationsInput { providers },
        };

        let operation = SimpleIntegrations::build(variables);
        let response = self.send_graphql_request(operation, None).await?;

        match response.simple_integrations {
            SimpleIntegrationsResult::SimpleIntegrationsOutput(output) => {
                Ok(simple_integrations_output(output))
            }
            SimpleIntegrationsResult::UserFacingError(error) => {
                Err(anyhow!(get_user_facing_error_message(error)))
            }
            SimpleIntegrationsResult::Unknown => {
                Err(anyhow!("Unknown error while listing simple integrations"))
            }
        }
    }

    async fn poll_oauth_connect_status(&self, tx_id: String) -> Result<OauthConnectTxStatus> {
        let variables = GetOAuthConnectTxStatusVariables {
            request_context: get_request_context(),
            input: GetOAuthConnectTxStatusInput {
                tx_id: cynic::Id::new(tx_id),
            },
        };

        let operation = GetOAuthConnectTxStatus::build(variables);
        let response = self.send_graphql_request(operation, None).await?;

        match response.get_oauth_connect_tx_status {
            GetOAuthConnectTxStatusResult::GetOAuthConnectTxStatusOutput(output) => {
                Ok(oauth_connect_tx_status(output.status))
            }
            GetOAuthConnectTxStatusResult::UserFacingError(error) => {
                Err(anyhow!(get_user_facing_error_message(error)))
            }
            GetOAuthConnectTxStatusResult::Unknown => {
                Err(anyhow!("Unknown error while polling OAuth status"))
            }
        }
    }

    async fn get_user_github_info(&self) -> Result<UserGithubInfoResult> {
        let variables = UserGithubInfoVariables {
            request_context: get_request_context(),
        };

        let operation = UserGithubInfo::build(variables);
        let response = self.send_graphql_request(operation, None).await?;

        let result = user_github_info_result(response.user_github_info);

        // Dev-only helper for testing GitHub-unauthed flows.
        //
        // Important: this runs after the network request completes so the UI can still
        // show the loading state.
        if FeatureFlag::SimulateGithubUnauthed.is_enabled() {
            if let UserGithubInfoResult::GithubConnectedOutput(connected) = &result {
                let auth_url = format!("{}/oauth/connect/github", ChannelState::server_root_url());
                return Ok(UserGithubInfoResult::GithubAuthRequiredOutput(
                    GithubAuthRequiredOutput {
                        auth_url,
                        // This value is unused by the app UI; it exists in the schema for
                        // tx-bound flows. We intentionally omit txId from the auth URL so
                        // the web flow can proceed without a server-created tx.
                        tx_id: "simulated".to_string(),
                        app_install_link: connected.app_install_link.clone(),
                    },
                ));
            }
        }

        Ok(result)
    }

    async fn suggest_cloud_environment_image(
        &self,
        repos: Vec<(String, String)>,
    ) -> Result<SuggestCloudEnvironmentImageResult> {
        let repo_inputs: Vec<SuggestCloudEnvironmentImageRepoInput> = repos
            .into_iter()
            .map(|(owner, repo)| SuggestCloudEnvironmentImageRepoInput { owner, repo })
            .collect();

        let variables = SuggestCloudEnvironmentImageVariables {
            request_context: get_request_context(),
            input: SuggestCloudEnvironmentImageInput { repos: repo_inputs },
        };

        let operation = SuggestCloudEnvironmentImage::build(variables);
        let response = self.send_graphql_request(operation, None).await?;

        match response.suggest_cloud_environment_image {
            warp_graphql::queries::suggest_cloud_environment_image::SuggestCloudEnvironmentImageResult::SuggestCloudEnvironmentImageAuthRequiredOutput(output) => Ok(SuggestCloudEnvironmentImageResult::SuggestCloudEnvironmentImageAuthRequiredOutput(
                    suggest_cloud_environment_image_auth_required_output(output),
                )),
            warp_graphql::queries::suggest_cloud_environment_image::SuggestCloudEnvironmentImageResult::SuggestCloudEnvironmentImageOutput(output) => {
                Ok(SuggestCloudEnvironmentImageResult::SuggestCloudEnvironmentImageOutput(
                    suggest_cloud_environment_image_output(output),
                ))
            }
            warp_graphql::queries::suggest_cloud_environment_image::SuggestCloudEnvironmentImageResult::UserFacingError(error) => {
                Err(anyhow!(get_user_facing_error_message(error)))
            }
            warp_graphql::queries::suggest_cloud_environment_image::SuggestCloudEnvironmentImageResult::Unknown => Err(anyhow!(
                "Unknown response from suggestCloudEnvironmentImage query"
            )),
        }
    }
}

#[cfg(not(feature = "oss_release"))]
fn user_repo_auth_status_output(
    output: warp_graphql::queries::user_repo_auth_status::UserRepoAuthStatusOutput,
) -> UserRepoAuthStatusOutput {
    UserRepoAuthStatusOutput {
        statuses: output
            .statuses
            .into_iter()
            .map(|status| UserRepoAuthStatus {
                owner: status.owner,
                repo: status.repo,
                status: user_repo_auth_status(status.status),
                is_public: status.is_public,
            })
            .collect(),
        auth_url: output.auth_url,
        tx_id: output.tx_id.map(|id| id.into_inner()),
    }
}

#[cfg(not(feature = "oss_release"))]
fn user_repo_auth_status(
    status: warp_graphql::queries::user_repo_auth_status::UserRepoAuthStatusEnum,
) -> UserRepoAuthStatusEnum {
    match status {
        warp_graphql::queries::user_repo_auth_status::UserRepoAuthStatusEnum::NoInstallationOrAccessForRepo => {
            UserRepoAuthStatusEnum::NoInstallationOrAccessForRepo
        }
        warp_graphql::queries::user_repo_auth_status::UserRepoAuthStatusEnum::UserNotConnectedToGithub => {
            UserRepoAuthStatusEnum::UserNotConnectedToGithub
        }
        warp_graphql::queries::user_repo_auth_status::UserRepoAuthStatusEnum::Success => {
            UserRepoAuthStatusEnum::Success
        }
    }
}

#[cfg(not(feature = "oss_release"))]
fn create_simple_integration_output(
    output: warp_graphql::mutations::create_simple_integration::CreateSimpleIntegrationOutput,
) -> CreateSimpleIntegrationOutput {
    CreateSimpleIntegrationOutput {
        auth_url: output.auth_url,
        success: output.success,
        message: output.message,
        tx_id: output.tx_id.map(|id| id.into_inner()),
    }
}

#[cfg(not(feature = "oss_release"))]
fn simple_integrations_output(
    output: warp_graphql::queries::get_simple_integrations::SimpleIntegrationsOutput,
) -> SimpleIntegrationsOutput {
    SimpleIntegrationsOutput {
        integrations: output
            .integrations
            .into_iter()
            .map(simple_integration)
            .collect(),
        message: output.message,
    }
}

#[cfg(not(feature = "oss_release"))]
fn simple_integration(
    integration: warp_graphql::queries::get_simple_integrations::SimpleIntegration,
) -> SimpleIntegration {
    SimpleIntegration {
        provider_slug: integration.provider_slug,
        description: integration.description,
        connection_status: simple_integration_connection_status(integration.connection_status),
        integration_config: integration
            .integration_config
            .map(|config| ListedSimpleIntegrationConfig {
                environment_uid: config.environment_uid,
                base_prompt: config.base_prompt,
                model_id: config.model_id,
                mcp_servers_json: config.mcp_servers_json,
            }),
        created_at: integration
            .created_at
            .map(|timestamp| ServerTimestamp::from_unix_timestamp_micros(timestamp.timestamp_micros()).expect("GraphQL timestamp should round-trip through microseconds")),
        updated_at: integration
            .updated_at
            .map(|timestamp| ServerTimestamp::from_unix_timestamp_micros(timestamp.timestamp_micros()).expect("GraphQL timestamp should round-trip through microseconds")),
    }
}

#[cfg(not(feature = "oss_release"))]
fn simple_integration_connection_status(
    status: warp_graphql::queries::get_simple_integrations::SimpleIntegrationConnectionStatus,
) -> SimpleIntegrationConnectionStatus {
    match status {
        warp_graphql::queries::get_simple_integrations::SimpleIntegrationConnectionStatus::NotConnected => {
            SimpleIntegrationConnectionStatus::NotConnected
        }
        warp_graphql::queries::get_simple_integrations::SimpleIntegrationConnectionStatus::ConnectionError => {
            SimpleIntegrationConnectionStatus::ConnectionError
        }
        warp_graphql::queries::get_simple_integrations::SimpleIntegrationConnectionStatus::IntegrationNotConfigured => {
            SimpleIntegrationConnectionStatus::IntegrationNotConfigured
        }
        warp_graphql::queries::get_simple_integrations::SimpleIntegrationConnectionStatus::NotEnabled => {
            SimpleIntegrationConnectionStatus::NotEnabled
        }
        warp_graphql::queries::get_simple_integrations::SimpleIntegrationConnectionStatus::Active => {
            SimpleIntegrationConnectionStatus::Active
        }
    }
}

#[cfg(not(feature = "oss_release"))]
fn oauth_connect_tx_status(
    status: warp_graphql::queries::get_oauth_connect_tx_status::OauthConnectTxStatus,
) -> OauthConnectTxStatus {
    match status {
        warp_graphql::queries::get_oauth_connect_tx_status::OauthConnectTxStatus::Completed => {
            OauthConnectTxStatus::Completed
        }
        warp_graphql::queries::get_oauth_connect_tx_status::OauthConnectTxStatus::Expired => {
            OauthConnectTxStatus::Expired
        }
        warp_graphql::queries::get_oauth_connect_tx_status::OauthConnectTxStatus::Failed => {
            OauthConnectTxStatus::Failed
        }
        warp_graphql::queries::get_oauth_connect_tx_status::OauthConnectTxStatus::InProgress => {
            OauthConnectTxStatus::InProgress
        }
        warp_graphql::queries::get_oauth_connect_tx_status::OauthConnectTxStatus::Pending => {
            OauthConnectTxStatus::Pending
        }
    }
}

#[cfg(not(feature = "oss_release"))]
fn user_github_info_result(
    result: warp_graphql::queries::user_github_info::UserGithubInfoResult,
) -> UserGithubInfoResult {
    match result {
        warp_graphql::queries::user_github_info::UserGithubInfoResult::GithubConnectedOutput(output) => {
            UserGithubInfoResult::GithubConnectedOutput(GithubConnectedOutput {
                username: output.username,
                installed_repos: output
                    .installed_repos
                    .into_iter()
                    .map(|repo| GithubRepoResult {
                        owner: repo.owner,
                        repo: repo.repo,
                        is_public: repo.is_public,
                    })
                    .collect(),
                app_install_link: output.app_install_link,
            })
        }
        warp_graphql::queries::user_github_info::UserGithubInfoResult::GithubAuthRequiredOutput(output) => {
            UserGithubInfoResult::GithubAuthRequiredOutput(GithubAuthRequiredOutput {
                auth_url: output.auth_url,
                tx_id: output.tx_id.into_inner(),
                app_install_link: output.app_install_link,
            })
        }
        warp_graphql::queries::user_github_info::UserGithubInfoResult::Unknown => {
            UserGithubInfoResult::Unknown
        }
    }
}

#[cfg(not(feature = "oss_release"))]
fn suggest_cloud_environment_image_auth_required_output(
    output: warp_graphql::queries::suggest_cloud_environment_image::SuggestCloudEnvironmentImageAuthRequiredOutput,
) -> SuggestCloudEnvironmentImageAuthRequiredOutput {
    SuggestCloudEnvironmentImageAuthRequiredOutput {
        auth_url: output.auth_url,
        tx_id: output.tx_id.into_inner(),
    }
}

#[cfg(not(feature = "oss_release"))]
fn suggest_cloud_environment_image_output(
    output: warp_graphql::queries::suggest_cloud_environment_image::SuggestCloudEnvironmentImageOutput,
) -> SuggestCloudEnvironmentImageOutput {
    SuggestCloudEnvironmentImageOutput {
        detected_languages: output
            .detected_languages
            .into_iter()
            .map(|language| GithubReposLanguageStat {
                bytes: language.bytes,
                language: language.language,
                percentage: language.percentage,
            })
            .collect(),
        image: output.image,
        needs_custom_image: output.needs_custom_image,
        reason: output.reason,
    }
}

#[cfg(feature = "oss_release")]
fn integrations_unavailable<T>() -> Result<T> {
    Err(anyhow!(
        "Integration server APIs are not available in this build"
    ))
}

#[cfg_attr(target_family = "wasm", async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg(feature = "oss_release")]
impl IntegrationsClient for ServerApi {
    async fn check_user_repo_auth_status(
        &self,
        _repos: Vec<(String, String)>,
    ) -> Result<UserRepoAuthStatusOutput> {
        Ok(UserRepoAuthStatusOutput {
            statuses: Vec::new(),
            auth_url: None,
            tx_id: None,
        })
    }

    #[allow(clippy::too_many_arguments)]
    async fn create_or_update_simple_integration(
        &self,
        _integration_type: String,
        _is_update: bool,
        _environment_uid: Option<String>,
        _base_prompt: Option<String>,
        _model_id: Option<String>,
        _mcp_servers_json: Option<String>,
        _remove_mcp_server_names: Option<Vec<String>>,
        _worker_host: Option<String>,
        _enabled: bool,
    ) -> Result<CreateSimpleIntegrationOutput> {
        integrations_unavailable()
    }

    async fn list_simple_integrations(
        &self,
        _providers: Vec<String>,
    ) -> Result<SimpleIntegrationsOutput> {
        Ok(SimpleIntegrationsOutput {
            integrations: Vec::new(),
            message: None,
        })
    }

    async fn poll_oauth_connect_status(&self, _tx_id: String) -> Result<OauthConnectTxStatus> {
        Ok(OauthConnectTxStatus::Failed)
    }

    async fn get_integrations_using_environment(
        &self,
        _environment_id: String,
    ) -> Result<GetIntegrationsUsingEnvironmentOutput> {
        Ok(GetIntegrationsUsingEnvironmentOutput {
            provider_names: Vec::new(),
        })
    }

    async fn get_user_github_info(&self) -> Result<UserGithubInfoResult> {
        Ok(UserGithubInfoResult::Unknown)
    }

    async fn suggest_cloud_environment_image(
        &self,
        _repos: Vec<(String, String)>,
    ) -> Result<SuggestCloudEnvironmentImageResult> {
        Ok(SuggestCloudEnvironmentImageResult::Unknown)
    }
}
