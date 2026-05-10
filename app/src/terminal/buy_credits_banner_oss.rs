use crate::server::ids::ServerId;
use warpui::{AppContext, Element, Entity, View, ViewContext, elements::Empty};

pub struct BuyCreditsBanner;

impl BuyCreditsBanner {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn is_denomination_dropdown_open(&self, _app: &AppContext) -> bool {
        false
    }
}

#[derive(Clone, Debug)]
pub enum BuyCreditsBannerEvent {
    OpenBillingAndUsage,
    RefocusInput,
    OpenAutoReloadModal { purchased_credits: i32 },
    ShowAutoReloadError { error_message: &'static str },
}

impl Entity for BuyCreditsBanner {
    type Event = BuyCreditsBannerEvent;
}

impl View for BuyCreditsBanner {
    fn ui_name() -> &'static str {
        "BuyCreditsBanner"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

#[derive(Clone, Debug)]
pub enum Action {
    SelectDenomination(usize),
    Close,
    PurchaseAddonCredits { team_uid: ServerId },
    ManageBilling,
    ToggleAutoReload,
}

impl warpui::TypedActionView for BuyCreditsBanner {
    type Action = Action;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}
