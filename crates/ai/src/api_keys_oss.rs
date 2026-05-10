pub use crate::aws_credentials::{AwsCredentials, AwsCredentialsState};
use warpui::{Entity, ModelContext, SingletonEntity};

#[derive(Clone, PartialEq, Eq)]
pub enum ApiKeyManagerEvent {
    KeysUpdated,
}

#[derive(Clone, PartialEq, Eq, Default)]
pub struct ApiKeys {
    pub google: Option<String>,
    pub anthropic: Option<String>,
    pub openai: Option<String>,
    pub open_router: Option<String>,
}

impl ApiKeys {
    pub fn has_any_key(&self) -> bool {
        false
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
pub enum AwsCredentialsRefreshStrategy {
    #[default]
    LocalChain,
    OidcManaged {
        task_id: Option<String>,
        role_arn: String,
    },
}

pub struct ApiKeyManager {
    keys: ApiKeys,
    aws_credentials_state: AwsCredentialsState,
    aws_credentials_refresh_strategy: AwsCredentialsRefreshStrategy,
}

impl ApiKeyManager {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self {
            keys: ApiKeys::default(),
            aws_credentials_state: AwsCredentialsState::Disabled,
            aws_credentials_refresh_strategy: AwsCredentialsRefreshStrategy::default(),
        }
    }

    pub fn keys(&self) -> &ApiKeys {
        &self.keys
    }

    pub fn set_google_key(&mut self, _key: Option<String>, ctx: &mut ModelContext<Self>) {
        ctx.emit(ApiKeyManagerEvent::KeysUpdated);
    }

    pub fn set_anthropic_key(&mut self, _key: Option<String>, ctx: &mut ModelContext<Self>) {
        ctx.emit(ApiKeyManagerEvent::KeysUpdated);
    }

    pub fn set_openai_key(&mut self, _key: Option<String>, ctx: &mut ModelContext<Self>) {
        ctx.emit(ApiKeyManagerEvent::KeysUpdated);
    }

    pub fn set_open_router_key(&mut self, _key: Option<String>, ctx: &mut ModelContext<Self>) {
        ctx.emit(ApiKeyManagerEvent::KeysUpdated);
    }

    pub fn set_aws_credentials_state(
        &mut self,
        state: AwsCredentialsState,
        ctx: &mut ModelContext<Self>,
    ) {
        self.aws_credentials_state = state;
        ctx.emit(ApiKeyManagerEvent::KeysUpdated);
    }

    pub fn aws_credentials_state(&self) -> &AwsCredentialsState {
        &self.aws_credentials_state
    }

    pub fn aws_credentials_refresh_strategy(&self) -> AwsCredentialsRefreshStrategy {
        self.aws_credentials_refresh_strategy.clone()
    }

    pub fn set_aws_credentials_refresh_strategy(
        &mut self,
        strategy: AwsCredentialsRefreshStrategy,
    ) {
        self.aws_credentials_refresh_strategy = strategy;
    }

    pub fn api_keys_for_request(
        &self,
        _include_byo_keys: bool,
        _include_aws_bedrock_credentials: bool,
    ) -> Option<warp_multi_agent_api::request::settings::ApiKeys> {
        None
    }
}

impl Entity for ApiKeyManager {
    type Event = ApiKeyManagerEvent;
}

impl SingletonEntity for ApiKeyManager {}
