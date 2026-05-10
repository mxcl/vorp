use crate::view_components::ToastFlavor;
use warpui::{elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum BuildPlanMigrationModalViewAction {
    SelectReloadDenomination(usize),
    EnableAutoReloadToggled(bool),
    GetStartedClicked,
    Close,
    OpenUrl(&'static str),
}

pub struct BuildPlanMigrationModal;

impl BuildPlanMigrationModal {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }
}

impl Entity for BuildPlanMigrationModal {
    type Event = BuildPlanMigrationModalEvent;
}

impl View for BuildPlanMigrationModal {
    fn ui_name() -> &'static str {
        "BuildPlanMigrationModal"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for BuildPlanMigrationModal {
    type Action = BuildPlanMigrationModalViewAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

#[derive(Clone, Debug)]
pub enum BuildPlanMigrationModalEvent {
    Close,
    ShowToast {
        message: String,
        flavor: ToastFlavor,
    },
}
