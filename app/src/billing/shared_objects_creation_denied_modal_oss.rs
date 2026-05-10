use crate::drive::DriveObjectType;
use crate::server::ids::ServerId;
use crate::workspaces::workspace::CustomerType;
use warpui::{
    elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext,
};

pub struct SharedObjectsCreationDeniedModal;

#[derive(Debug)]
pub enum SharedObjectsCreationDeniedModalAction {
    Close,
}

pub enum SharedObjectsCreationDeniedModalEvent {
    Close,
    TeamSettings,
}

pub fn init(_app: &mut AppContext) {}

impl SharedObjectsCreationDeniedModal {
    pub fn new(_object_type: Option<DriveObjectType>, _ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn update_modal_state(
        &mut self,
        _team_uid: ServerId,
        _object_type: DriveObjectType,
        _has_admin_permissions: bool,
        _is_delinquent_due_to_payment_issue: bool,
        _customer_type: CustomerType,
        _ctx: &mut ViewContext<Self>,
    ) {
    }
}

impl Entity for SharedObjectsCreationDeniedModal {
    type Event = SharedObjectsCreationDeniedModalEvent;
}

impl View for SharedObjectsCreationDeniedModal {
    fn ui_name() -> &'static str {
        "SharedObjectsCreationDeniedModal"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for SharedObjectsCreationDeniedModal {
    type Action = SharedObjectsCreationDeniedModalAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}
