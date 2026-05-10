use std::sync::Arc;

use warpui::{Entity, ModelContext, SingletonEntity};

use super::auth_view_modal::{AuthRedirectPayload, AuthViewVariant};
use crate::server::server_api::{
    auth::{AuthClient, MintCustomTokenError, UserAuthenticationError},
    ServerApi,
};
use crate::server::telemetry::AnonymousUserSignupEntrypoint;

pub enum AuthManagerEvent {
    AuthComplete,
    AuthFailed(UserAuthenticationError),
    CreateAnonymousUserFailed,
    SkippedLogin,
    NeedsReauth,
    AttemptedLoginGatedFeature {
        auth_view_variant: AuthViewVariant,
    },
    LoginOverrideDetected(AuthRedirectPayload),
    MintCustomTokenFailed(MintCustomTokenError),
    ReceivedDeviceAuthorizationCode {
        verification_url: String,
        verification_url_complete: Option<String>,
        user_code: String,
    },
}

impl std::fmt::Debug for AuthManagerEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AuthComplete => f.write_str("AuthComplete"),
            Self::AuthFailed(_) => f.write_str("AuthFailed"),
            Self::CreateAnonymousUserFailed => f.write_str("CreateAnonymousUserFailed"),
            Self::SkippedLogin => f.write_str("SkippedLogin"),
            Self::NeedsReauth => f.write_str("NeedsReauth"),
            Self::AttemptedLoginGatedFeature { .. } => f.write_str("AttemptedLoginGatedFeature"),
            Self::LoginOverrideDetected(_) => f.write_str("LoginOverrideDetected"),
            Self::MintCustomTokenFailed(_) => f.write_str("MintCustomTokenFailed"),
            Self::ReceivedDeviceAuthorizationCode { .. } => {
                f.write_str("ReceivedDeviceAuthorizationCode")
            }
        }
    }
}

pub type LoginGatedFeature = &'static str;

type URLConstructorCallback = Box<dyn FnOnce(Option<&str>) -> String>;

pub struct AuthManager;

impl AuthManager {
    pub fn new(
        _server_api: Arc<ServerApi>,
        _auth_client: Arc<dyn AuthClient>,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self
    }

    #[cfg(test)]
    pub fn new_for_test(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }

    pub fn initialize_user_from_auth_payload(
        &mut self,
        _auth_payload: AuthRedirectPayload,
        _enforce_state_validation: bool,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn resume_interrupted_auth_payload(
        &mut self,
        _auth_payload: AuthRedirectPayload,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    #[cfg(target_family = "wasm")]
    pub fn initialize_user_from_session_cookie(&self, _ctx: &mut ModelContext<Self>) {}

    pub fn refresh_user(&self, _ctx: &mut ModelContext<Self>) {}

    pub fn authorize_device(&self, _ctx: &mut ModelContext<Self>) {}

    pub(super) fn log_out(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn set_needs_reauth(&self, _needs_reauth: bool, _ctx: &mut ModelContext<Self>) {}

    pub fn create_anonymous_user(
        &self,
        _referral_code: Option<String>,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn attempt_login_gated_feature(
        &self,
        _feature: LoginGatedFeature,
        _auth_view_variant: AuthViewVariant,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn anonymous_user_hit_drive_object_limit(&self, _ctx: &mut ModelContext<Self>) {}

    pub fn initiate_anonymous_user_linking(
        &self,
        _entrypoint: AnonymousUserSignupEntrypoint,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn open_url_maybe_with_anonymous_token(
        &self,
        _ctx: &mut ModelContext<Self>,
        _construct_url: URLConstructorCallback,
    ) {
    }

    pub fn copy_anonymous_user_linking_url_to_clipboard(&self, _ctx: &mut ModelContext<Self>) {}

    pub fn sign_up_url(&mut self) -> String {
        "about:blank".to_owned()
    }

    pub fn sign_in_url(&mut self) -> String {
        "about:blank".to_owned()
    }

    pub fn upgrade_url(&mut self) -> String {
        "about:blank".to_owned()
    }

    pub fn login_options_url(&mut self, _custom_token: &str) -> String {
        "about:blank".to_owned()
    }

    pub fn link_sso_url(&mut self, _email: &str) -> String {
        "about:blank".to_owned()
    }

    pub fn set_user_onboarded(&self, _ctx: &mut ModelContext<Self>) {}
}

#[derive(Clone, Debug)]
pub struct PersistedCurrentUserInformation {
    pub email: String,
}

impl Entity for AuthManager {
    type Event = AuthManagerEvent;
}

impl SingletonEntity for AuthManager {}
