use crate::settings_view::{
    settings_page::{MatchData, SettingsPageMeta, SettingsPageViewHandle},
    SettingsSection,
};
use warpui::{
    elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext, ViewHandle,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum InstallOrigin {
    InApp,
    Deeplink,
}

impl std::fmt::Debug for InstallOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("InstallOrigin")
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ServerCardItemId {}

impl std::fmt::Debug for ServerCardItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {}
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum MCPServersSettingsPage {
    #[default]
    List,
    Edit {
        item_id: Option<ServerCardItemId>,
    },
}

impl std::fmt::Debug for MCPServersSettingsPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SettingsPage")
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum MCPServersSettingsPageEvent {
    ShowModal,
    HideModal,
}

impl std::fmt::Debug for MCPServersSettingsPageEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SettingsPageEvent")
    }
}

pub struct MCPServersSettingsPageView;

impl MCPServersSettingsPageView {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn update_page(&mut self, _page: MCPServersSettingsPage, _ctx: &mut ViewContext<Self>) {}

    pub fn autoinstall_from_gallery(&mut self, _title: &str, _ctx: &mut ViewContext<Self>) {}

    pub fn get_modal_content(&self, _app: &AppContext) -> Option<Box<dyn Element>> {
        None
    }

    pub fn should_show_install_modal(
        _origin: InstallOrigin,
        _already_installed: bool,
        _is_shared: bool,
    ) -> bool {
        false
    }
}

impl Entity for MCPServersSettingsPageView {
    type Event = MCPServersSettingsPageEvent;
}

impl View for MCPServersSettingsPageView {
    fn ui_name() -> &'static str {
        "SettingsPageView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for MCPServersSettingsPageView {
    type Action = ();
}

impl SettingsPageMeta for MCPServersSettingsPageView {
    fn section() -> SettingsSection {
        SettingsSection::MCPServers
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
