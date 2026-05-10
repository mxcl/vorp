use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, TypedActionView, View, ViewContext};

use crate::server::ids::ServerId;

#[derive(Clone, PartialEq, Eq)]
pub enum PromptAlertAction {
    SignUpClickedForAnonymousUser,
    OpenSettingsClicked,
    OpenPrivacySettingsClicked,
    ManageBillingClicked { team_uid: ServerId },
}

impl std::fmt::Debug for PromptAlertAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Action")
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum PromptAlertEvent {
    SignupAnonymousUser,
    OpenBillingAndUsagePage,
    OpenPrivacyPage,
    OpenBillingPortal { team_uid: ServerId },
}

impl std::fmt::Debug for PromptAlertEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Event")
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum PromptAlertState {
    NoConnection,
    TelemetryDisabledOnFreeTier,
    AnonymousUserRequestLimitSoftGate,
    AnonymousUserRequestLimitHardGate,
    DelinquentDueToPaymentIssue,
    OveragesToggleableButNotEnabled,
    MonthlyOveragesSpendLimitReached,
    RequestLimitReached,
    NoAlert,
}

impl std::fmt::Debug for PromptAlertState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("State")
    }
}

pub type PromptAlertView = AlertView;

pub struct AlertView {
    state: PromptAlertState,
}

impl AlertView {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self {
            state: PromptAlertState::NoAlert,
        }
    }

    pub fn determine_state(_app: &AppContext) -> PromptAlertState {
        PromptAlertState::NoAlert
    }

    pub fn is_no_alert(&self) -> bool {
        true
    }

    pub fn state(&self) -> &PromptAlertState {
        &self.state
    }

    pub fn does_alert_block_ai_requests(_app: &AppContext) -> bool {
        false
    }
}

impl Entity for AlertView {
    type Event = PromptAlertEvent;
}

impl View for AlertView {
    fn ui_name() -> &'static str {
        "AlertView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for AlertView {
    type Action = PromptAlertAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            PromptAlertAction::SignUpClickedForAnonymousUser => {
                ctx.emit(PromptAlertEvent::SignupAnonymousUser);
            }
            PromptAlertAction::OpenSettingsClicked => {
                ctx.emit(PromptAlertEvent::OpenBillingAndUsagePage);
            }
            PromptAlertAction::OpenPrivacySettingsClicked => {
                ctx.emit(PromptAlertEvent::OpenPrivacyPage);
            }
            PromptAlertAction::ManageBillingClicked { team_uid } => {
                ctx.emit(PromptAlertEvent::OpenBillingPortal {
                    team_uid: *team_uid,
                });
            }
        }
    }
}
