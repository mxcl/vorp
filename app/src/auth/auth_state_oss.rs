use std::sync::Arc;

use warpui::{AppContext, Entity, SingletonEntity};

use super::{
    anonymous_id::get_or_create_anonymous_id,
    credentials::{AuthOwnerType, Credentials},
    user::{AnonymousUserType, FirebaseAuthTokens, PersonalObjectLimits, PrincipalType, User},
    UserUid,
};
use crate::cloud_object::ObjectType;

pub struct AuthState {
    anonymous_id: String,
}

impl AuthState {
    fn new(ctx: &AppContext) -> Self {
        Self {
            anonymous_id: get_or_create_anonymous_id(ctx).to_string(),
        }
    }

    #[cfg(any(test, feature = "integration_tests"))]
    pub fn new_for_test() -> Self {
        Self {
            anonymous_id: uuid::Uuid::new_v4().to_string(),
        }
    }

    pub fn initialize(ctx: &AppContext, _api_key: Option<String>) -> Self {
        Self::new(ctx)
    }

    pub(super) fn set_user(&self, _user: Option<User>) {}

    pub fn credentials(&self) -> Option<Credentials> {
        None
    }

    pub(super) fn set_credentials(&self, _credentials: Option<Credentials>) {}

    pub(crate) fn update_firebase_tokens(&self, _new_auth_tokens: FirebaseAuthTokens) {}

    pub fn is_logged_in(&self) -> bool {
        false
    }

    pub fn is_anonymous_or_logged_out(&self) -> bool {
        true
    }

    pub fn get_access_token_ignoring_validity(&self) -> Option<String> {
        None
    }

    pub fn username_for_display(&self) -> Option<String> {
        None
    }

    pub fn display_name(&self) -> Option<String> {
        None
    }

    pub fn user_email(&self) -> Option<String> {
        None
    }

    pub fn is_onboarded(&self) -> Option<bool> {
        None
    }

    pub fn user_email_domain(&self) -> Option<String> {
        None
    }

    pub fn is_user_anonymous(&self) -> Option<bool> {
        None
    }

    pub fn is_user_web_anonymous_user(&self) -> Option<bool> {
        None
    }

    pub fn is_anonymous_user_feature_gated(&self) -> Option<bool> {
        None
    }

    pub fn is_anonymous_user_past_object_limit(
        &self,
        _object_type: ObjectType,
        _num_objects: usize,
    ) -> Option<bool> {
        None
    }

    pub fn user_photo_url(&self) -> Option<String> {
        None
    }

    pub fn needs_sso_link(&self) -> Option<bool> {
        None
    }

    pub fn anonymous_user_type(&self) -> Option<AnonymousUserType> {
        None
    }

    pub fn personal_object_limits(&self) -> Option<PersonalObjectLimits> {
        None
    }

    pub fn set_is_onboarded(&self, _is_onboarded: bool) {}

    pub fn user_id(&self) -> Option<UserUid> {
        None
    }

    pub fn anonymous_id(&self) -> String {
        self.anonymous_id.clone()
    }

    pub fn needs_reauth(&self) -> bool {
        false
    }

    pub(super) fn set_needs_reauth(&self, _new_needs_reauth: bool) -> bool {
        false
    }

    pub fn anonymous_user_renotification_block_expired(
        &self,
        _last_time_opt: Option<String>,
    ) -> bool {
        false
    }

    pub fn is_on_work_domain(&self) -> Option<bool> {
        None
    }

    pub fn is_api_key_authenticated(&self) -> bool {
        false
    }

    pub fn api_key(&self) -> Option<String> {
        None
    }

    pub fn principal_type(&self) -> Option<PrincipalType> {
        None
    }

    pub fn is_service_account(&self) -> bool {
        false
    }

    pub fn api_key_owner_type(&self) -> Option<AuthOwnerType> {
        None
    }
}

impl crate::warp_managed_secrets::ActorProvider for AuthState {
    fn actor_uid(&self) -> Option<String> {
        None
    }
}

pub struct AuthStateProvider {
    auth_state: Arc<AuthState>,
}

impl AuthStateProvider {
    pub fn new(auth_state: Arc<AuthState>) -> Self {
        Self { auth_state }
    }

    #[cfg(test)]
    pub fn new_for_test() -> Self {
        Self {
            auth_state: Arc::new(AuthState::new_for_test()),
        }
    }

    #[cfg(test)]
    pub fn new_logged_out_for_test() -> Self {
        Self {
            auth_state: Arc::new(AuthState::new_for_test()),
        }
    }

    pub fn get(&self) -> &Arc<AuthState> {
        &self.auth_state
    }
}

impl Entity for AuthStateProvider {
    type Event = ();
}

impl SingletonEntity for AuthStateProvider {}
