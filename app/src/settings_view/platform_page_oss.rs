use super::{
    settings_page::{MatchData, SettingsPageMeta, SettingsPageViewHandle},
    SettingsSection,
};
use warpui::{
    elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext, ViewHandle,
};

#[derive(Clone, Copy)]
pub enum PlatformPageViewEvent {
    ShowCreateApiKeyModal,
    HideCreateApiKeyModal,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlatformPageAction {
    ShowCreateApiKeyModal,
    HyperlinkClick(String),
}

pub struct PlatformPageView;

impl PlatformPageView {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn get_modal_content(&self) -> Option<Box<dyn Element>> {
        None
    }
}

impl Entity for PlatformPageView {
    type Event = PlatformPageViewEvent;
}

impl View for PlatformPageView {
    fn ui_name() -> &'static str {
        "PlatformPageView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for PlatformPageView {
    type Action = PlatformPageAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl SettingsPageMeta for PlatformPageView {
    fn section() -> SettingsSection {
        SettingsSection::OzCloudAPIKeys
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

impl From<ViewHandle<PlatformPageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<PlatformPageView>) -> Self {
        SettingsPageViewHandle::OzCloudAPIKeys(view_handle)
    }
}
