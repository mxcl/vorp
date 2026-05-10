#[cfg(not(feature = "oss_release"))]
use super::auth::AuthClient;
use super::ServerApi;
use crate::ai::generate_block_title::api::{GenerateBlockTitleRequest, GenerateBlockTitleResponse};
use crate::server::block::{Block, DisplaySetting};
#[cfg(not(feature = "oss_release"))]
use crate::server::graphql::{get_request_context, get_user_facing_error_message};
use anyhow::anyhow;
use async_trait::async_trait;
#[cfg(not(feature = "oss_release"))]
use chrono::Utc;
#[cfg(not(feature = "oss_release"))]
use cynic::{MutationBuilder, QueryBuilder};
#[cfg(test)]
use mockall::automock;
#[cfg(not(feature = "oss_release"))]
use std::convert::TryFrom;
#[cfg(not(feature = "oss_release"))]
use warp_core::channel::{Channel, ChannelState};
#[cfg(not(feature = "oss_release"))]
use warp_graphql::{
    mutations::{
        share_block::{BlockInput, ShareBlock, ShareBlockResult, ShareBlockVariables},
        unshare_block::{
            UnshareBlock, UnshareBlockInput, UnshareBlockResult, UnshareBlockVariables,
        },
    },
    queries::get_blocks_for_user::{
        Block as GqlBlock, GetBlocksForUser, GetBlocksForUserVariables,
    },
};

#[cfg_attr(test, automock)]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait BlockClient: 'static + Send + Sync {
    /// Unshares a block identified at `block_id`.
    async fn unshare_block(&self, block_id: String) -> Result<(), anyhow::Error>;

    /// Uploads a given block to the server via the /share_block endpoint.
    async fn save_block(
        &self,
        block: &Block,
        title: Option<String>,
        show_prompt: bool,
        display_setting: DisplaySetting,
    ) -> Result<String, anyhow::Error>;

    async fn blocks_owned_by_user(&self) -> Result<Vec<Block>, anyhow::Error>;

    async fn generate_shared_block_title(
        &self,
        request: GenerateBlockTitleRequest,
    ) -> Result<GenerateBlockTitleResponse, anyhow::Error>;
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
#[cfg(not(feature = "oss_release"))]
impl BlockClient for ServerApi {
    async fn unshare_block(&self, block_uid: String) -> Result<(), anyhow::Error> {
        let variables = UnshareBlockVariables {
            input: UnshareBlockInput { block_uid },
            request_context: get_request_context(),
        };

        let operation = UnshareBlock::build(variables);
        let response = self.send_graphql_request(operation, None).await?;
        match response.unshare_block {
            UnshareBlockResult::UnshareBlockOutput(output) => {
                if output.success {
                    Ok(())
                } else {
                    Err(anyhow!("Failed to unshare block"))
                }
            }
            UnshareBlockResult::UserFacingError(error) => {
                Err(anyhow!(get_user_facing_error_message(error)))
            }
            UnshareBlockResult::Unknown => Err(anyhow!("Failed to unshare block")),
        }
    }

    async fn save_block(
        &self,
        block: &Block,
        title: Option<String>,
        show_prompt: bool,
        display_setting: DisplaySetting,
    ) -> Result<String, anyhow::Error> {
        let variables = ShareBlockVariables {
            block: BlockInput {
                command: block.command.as_deref(),
                embed_display_setting: display_setting.into(),
                output: block.output.as_deref(),
                show_prompt,
                stylized_command: block.stylized_command.as_deref(),
                stylized_output: block.stylized_output.as_deref(),
                stylized_prompt: block.stylized_prompt.as_deref(),
                stylized_prompt_and_command: block.stylized_prompt_and_command.as_deref(),
                time_started_term: Some(block.time_started_term.with_timezone(&Utc).into()),
                title: title.as_deref(),
            },
            request_context: get_request_context(),
        };

        let operation = ShareBlock::build(variables);
        let response = self.send_graphql_request(operation, None).await?;
        match response.share_block {
            ShareBlockResult::ShareBlockOutput(output) => {
                let mut created_url =
                    format!("{}{}", ChannelState::server_root_url(), output.url_ending);

                // If this is a preview build, ensure the link routes to a preview build.
                if matches!(ChannelState::channel(), Channel::Preview) {
                    created_url.push_str("?preview=true");
                }

                Ok(created_url)
            }
            ShareBlockResult::UserFacingError(error) => {
                Err(anyhow!(get_user_facing_error_message(error)))
            }
            ShareBlockResult::Unknown => Err(anyhow!("Failed to share block")),
        }
    }

    async fn blocks_owned_by_user(&self) -> Result<Vec<Block>, anyhow::Error> {
        let variables = GetBlocksForUserVariables {
            request_context: get_request_context(),
        };
        let operation = GetBlocksForUser::build(variables);
        let response = self.send_graphql_request(operation, None).await?;

        match response.user {
            warp_graphql::queries::get_blocks_for_user::UserResult::UserOutput(user_output) => {
                Ok(user_output
                    .user
                    .blocks
                    .into_iter()
                    .filter_map(|block| block.try_into().ok())
                    .collect())
            }
            warp_graphql::queries::get_blocks_for_user::UserResult::Unknown => {
                Err(anyhow!("Unable to fetch blocks"))
            }
        }
    }

    async fn generate_shared_block_title(
        &self,
        request: GenerateBlockTitleRequest,
    ) -> Result<GenerateBlockTitleResponse, anyhow::Error> {
        let auth_token = self.get_or_refresh_access_token().await?;
        let request_builder = self.client.post(format!(
            "{}/ai/generate_block_title",
            ChannelState::server_root_url()
        ));
        let response = if let Some(token) = auth_token.as_bearer_token() {
            request_builder.bearer_auth(token)
        } else {
            request_builder
        }
        .json(&request)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
        Ok(response)
    }
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
#[cfg(feature = "oss_release")]
impl BlockClient for ServerApi {
    async fn unshare_block(&self, _block_uid: String) -> Result<(), anyhow::Error> {
        Ok(())
    }

    async fn save_block(
        &self,
        _block: &Block,
        _title: Option<String>,
        _show_prompt: bool,
        _display_setting: DisplaySetting,
    ) -> Result<String, anyhow::Error> {
        Err(anyhow!(
            "Block sharing server APIs are not available in this build"
        ))
    }

    async fn blocks_owned_by_user(&self) -> Result<Vec<Block>, anyhow::Error> {
        Ok(Vec::new())
    }

    async fn generate_shared_block_title(
        &self,
        _request: GenerateBlockTitleRequest,
    ) -> Result<GenerateBlockTitleResponse, anyhow::Error> {
        Err(anyhow!(
            "AI block title generation is not available in this build"
        ))
    }
}

#[cfg(not(feature = "oss_release"))]
impl TryFrom<GqlBlock> for Block {
    type Error = anyhow::Error;

    fn try_from(value: GqlBlock) -> Result<Self, Self::Error> {
        match (value.uid, value.time_started_term) {
            (uid, Some(time_started_term)) => {
                Ok(Block {
                    id: Some(uid.into_inner()),
                    command: value.command,
                    output: None,
                    stylized_command: None,
                    stylized_output: None,
                    pwd: None,
                    time_started_term: time_started_term.utc().into(),
                    // This is a dummy value - we are no longer using time_completed_term,
                    // and GqlBlock does not have a time_completed_term field.
                    time_completed_term: time_started_term.utc().into(),
                    stylized_prompt: None,
                    stylized_prompt_and_command: None,
                })
            }
            _ => Err(anyhow!("missing id or time_started_term")),
        }
    }
}
