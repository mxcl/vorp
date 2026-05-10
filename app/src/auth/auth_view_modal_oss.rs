use std::collections::HashMap;

use anyhow::{anyhow, Result};
use url::Url;
use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, TypedActionView, View, ViewContext};

use super::credentials::RefreshToken;
use super::login_failure_notification::LoginFailureReason;
use super::UserUid;

const AUTH_URL_HOST: &str = "auth";
const AUTH_URL_REFRESH_TOKEN_QUERY_PARAM: &str = "refresh_token";
const AUTH_URL_NEW_USER_UID_QUERY_PARAM: &str = "user_uid";
const AUTH_URL_DELETED_ANON_USER_QUERY_PARAM: &str = "deleted_anonymous_user";
const AUTH_URL_STATE_QUERY_PARAM: &str = "state";

#[derive(Clone, Debug)]
pub enum AuthViewAction {
    PasteAuthUrl,
    DismissErrorNotification,
}

#[derive(Debug, Clone)]
pub struct AuthRedirectPayload {
    pub refresh_token: RefreshToken,
    pub user_uid: Option<UserUid>,
    pub deleted_anonymous_user: Option<bool>,
    pub state: Option<String>,
}

impl AuthRedirectPayload {
    pub fn from_url(url: Url) -> Result<Self> {
        if url.host_str() != Some(AUTH_URL_HOST) {
            return Err(anyhow!("Received URL with unexpected host: {} ", url));
        }
        let query_params: HashMap<_, _> = url.query_pairs().into_owned().collect();
        if let Some(token) = query_params.get(AUTH_URL_REFRESH_TOKEN_QUERY_PARAM) {
            let user_uid = query_params
                .get(AUTH_URL_NEW_USER_UID_QUERY_PARAM)
                .map(|uid| UserUid::new(uid));

            Ok(Self {
                refresh_token: RefreshToken::new(token),
                user_uid,
                deleted_anonymous_user: query_params
                    .get(AUTH_URL_DELETED_ANON_USER_QUERY_PARAM)
                    .map(|value| value == "true"),
                state: query_params.get(AUTH_URL_STATE_QUERY_PARAM).cloned(),
            })
        } else {
            Err(anyhow!(
                "Received URL without refresh token query param: {}",
                url
            ))
        }
    }

    pub fn from_raw_url(raw_url: String) -> Result<Self> {
        match Url::parse(&raw_url) {
            Ok(parsed_url) => Self::from_url(parsed_url),
            Err(error) => Err(anyhow!(error)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AuthViewVariant {
    Initial,
    RequireLoginCloseable,
    HitDriveObjectLimitCloseable,
    ShareRequirementCloseable,
}

pub struct AuthView {
    pub last_login_failure_reason: Option<LoginFailureReason>,
    auth_view_variant: AuthViewVariant,
}

impl AuthView {
    pub fn new(variant: AuthViewVariant, _ctx: &mut ViewContext<Self>) -> Self {
        Self {
            last_login_failure_reason: None,
            auth_view_variant: variant,
        }
    }

    pub fn set_variant(&mut self, _ctx: &mut ViewContext<Self>, variant: AuthViewVariant) {
        self.auth_view_variant = variant;
    }

    pub fn skip_to_browser_open_step(&mut self, _ctx: &mut ViewContext<Self>) {}
}

#[derive(PartialEq, Eq)]
pub enum AuthViewEvent {
    Close,
}

impl Entity for AuthView {
    type Event = AuthViewEvent;
}

impl View for AuthView {
    fn ui_name() -> &'static str {
        "AuthView"
    }

    fn render(&self, _ctx: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for AuthView {
    type Action = AuthViewAction;

    fn handle_action(&mut self, _action: &AuthViewAction, _ctx: &mut ViewContext<Self>) {}
}
