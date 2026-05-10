use crate::settings_view::{
    settings_page::{MatchData, SettingsPageMeta, SettingsPageViewHandle},
    SettingsSection,
};
use crate::view_components::ToastFlavor;
use warp_core::ui::appearance::Appearance;
use warpui::{
    elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext, ViewHandle,
};

pub fn create_discount_badge(_discount: u32, _appearance: &Appearance) -> Box<dyn Element> {
    Empty::new().finish()
}

pub struct BillingAndUsagePageView;

impl BillingAndUsagePageView {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn get_modal_content(&self) -> Option<Box<dyn Element>> {
        None
    }
}

#[derive(Debug, Clone)]
pub enum BillingAndUsagePageEvent {
    SignupAnonymousUser,
    ShowToast {
        message: String,
        flavor: ToastFlavor,
    },
    ShowModal,
    HideModal,
}

impl Entity for BillingAndUsagePageView {
    type Event = BillingAndUsagePageEvent;
}

impl View for BillingAndUsagePageView {
    fn ui_name() -> &'static str {
        "Billing and usage"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

#[derive(Debug, Clone)]
pub enum BillingAndUsagePageAction {}

impl TypedActionView for BillingAndUsagePageView {
    type Action = BillingAndUsagePageAction;

    fn handle_action(&mut self, action: &Self::Action, _ctx: &mut ViewContext<Self>) {
        match *action {}
    }
}

impl SettingsPageMeta for BillingAndUsagePageView {
    fn section() -> SettingsSection {
        SettingsSection::BillingAndUsage
    }

    fn should_render(&self, _ctx: &AppContext) -> bool {
        false
    }

    fn update_filter(&mut self, _query: &str, _ctx: &mut ViewContext<Self>) -> MatchData {
        MatchData::Uncounted(false)
    }

    fn scroll_to_widget(&mut self, _widget_id: &'static str) {}

    fn clear_highlighted_widget(&mut self) {}
}

impl From<ViewHandle<BillingAndUsagePageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<BillingAndUsagePageView>) -> Self {
        SettingsPageViewHandle::BillingAndUsage(view_handle)
    }
}
