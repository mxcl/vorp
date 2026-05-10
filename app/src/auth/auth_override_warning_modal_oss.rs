use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, TypedActionView, View, ViewContext};

use crate::auth::auth_view_modal::AuthRedirectPayload;

pub struct AuthOverrideWarningModal;

pub enum AuthOverrideWarningModalVariant {
    OnboardingView,
    WorkspaceModal,
}

impl AuthOverrideWarningModal {
    pub fn new(_ctx: &mut ViewContext<Self>, _variant: AuthOverrideWarningModalVariant) -> Self {
        Self
    }

    pub fn set_interrupted_auth_payload(&mut self, _auth_payload: AuthRedirectPayload) {}
}

#[derive(PartialEq, Eq)]
pub enum AuthOverrideWarningModalEvent {
    Close,
    BulkExport,
}

impl Entity for AuthOverrideWarningModal {
    type Event = AuthOverrideWarningModalEvent;
}

impl View for AuthOverrideWarningModal {
    fn ui_name() -> &'static str {
        "AuthOverrideWarningModal"
    }

    fn render(&self, _ctx: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for AuthOverrideWarningModal {
    type Action = ();
}
