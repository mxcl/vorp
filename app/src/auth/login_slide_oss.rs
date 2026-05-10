use onboarding::OnboardingIntention;
use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, TypedActionView, View, ViewContext};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoginSlideSource {
    OnboardingFlow,
    LoginExistingUserFromWelcome,
    PrivacySettingsFromTerminalIntentionTheme,
}

#[derive(Clone, Debug)]
pub enum LoginSlideEvent {
    BackToOnboarding,
    LoginLaterConfirmed,
}

#[derive(Debug)]
pub enum LoginSlideAction {
    Enter,
    ShowSkipDialog,
    ConfirmSkip,
    DismissDialog,
    DismissOverlayOrBack,
    Back,
    BackToSelectAuthPathway,
    CopyLoginUrl,
    EnterToken,
    ShowPrivacySettings,
    HideOverlay,
    ToggleTelemetry,
    ToggleCrashReporting,
    DismissNotification,
    PasteAuthUrl,
}

pub struct LoginSlideView;

impl LoginSlideView {
    pub fn new(
        _ai_enabled: bool,
        _theme_name: &str,
        _use_vertical_tabs: bool,
        _intention: OnboardingIntention,
        _source: LoginSlideSource,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn is_auth_token_input_visible(&self) -> bool {
        false
    }
}

impl Entity for LoginSlideView {
    type Event = LoginSlideEvent;
}

impl View for LoginSlideView {
    fn ui_name() -> &'static str {
        "LoginSlideView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for LoginSlideView {
    type Action = LoginSlideAction;

    fn handle_action(&mut self, _action: &LoginSlideAction, _ctx: &mut ViewContext<Self>) {}
}
