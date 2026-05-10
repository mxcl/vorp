use crate::settings_view::{
    settings_page::{MatchData, SettingsPageMeta, SettingsPageViewHandle},
    SettingsSection,
};
use warpui::{
    elements::Empty, Action, AppContext, Element, Entity, TypedActionView, View, ViewContext,
    ViewHandle,
};

pub(crate) fn cli_agent_settings_widget_id() -> &'static str {
    ""
}

pub fn init_actions_from_parent_view<T: Action + Clone>(
    _app: &mut AppContext,
    _context: &warpui::keymap::ContextPredicate,
    _builder: fn(crate::settings_view::SettingsAction) -> T,
) {
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AISubpage {
    WarpAgent,
    Profiles,
    Knowledge,
    ThirdPartyCLIAgents,
}

impl std::fmt::Debug for AISubpage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SettingsSubpage")
    }
}

impl AISubpage {
    pub fn from_section(section: SettingsSection) -> Option<Self> {
        match section {
            SettingsSection::WarpAgent => Some(Self::WarpAgent),
            SettingsSection::AgentProfiles => Some(Self::Profiles),
            SettingsSection::Knowledge => Some(Self::Knowledge),
            SettingsSection::ThirdPartyCLIAgents => Some(Self::ThirdPartyCLIAgents),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub enum AISettingsPageAction {
    Noop,
}

impl std::fmt::Debug for AISettingsPageAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SettingsPageAction")
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum AISettingsPageEvent {
    FocusModal,
    OpenAIFactCollection,
    OpenMCPServerCollection,
    OpenExecutionProfileEditor(crate::ai::execution_profiles::profiles::ClientProfileId),
    SignupAnonymousUser,
}

impl std::fmt::Debug for AISettingsPageEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SettingsPageEvent")
    }
}

pub struct AISettingsPageView;

impl AISettingsPageView {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn set_active_subpage(
        &mut self,
        _subpage: Option<AISubpage>,
        _ctx: &mut ViewContext<Self>,
    ) {
    }
}

impl Entity for AISettingsPageView {
    type Event = AISettingsPageEvent;
}

impl View for AISettingsPageView {
    fn ui_name() -> &'static str {
        "SettingsPageView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for AISettingsPageView {
    type Action = AISettingsPageAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl SettingsPageMeta for AISettingsPageView {
    fn section() -> SettingsSection {
        SettingsSection::AI
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

impl From<ViewHandle<AISettingsPageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<AISettingsPageView>) -> Self {
        SettingsPageViewHandle::AI(view_handle)
    }
}
