use crate::{
    pane_group::{
        focus_state::PaneFocusHandle,
        pane::{
            view::{HeaderContent, HeaderRenderContext},
            BackingView, PaneConfiguration,
        },
    },
    server::ids::SyncId,
    settings_view::{
        settings_page::{MatchData, SettingsPageEvent, SettingsPageMeta, SettingsPageViewHandle},
        update_environment_form::{GithubAuthRedirectTarget, UpdateEnvironmentForm},
        SettingsSection,
    },
    terminal::view::init_environment::mode_selector::EnvironmentSetupModeSelector,
};
use warpui::{
    elements::Empty, AppContext, Element, Entity, ModelHandle, TypedActionView, View, ViewContext,
    ViewHandle,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub enum EnvironmentsPage {
    #[default]
    List,
    Edit {
        env_id: SyncId,
    },
    Create,
}

#[derive(Debug, Clone)]
pub enum EnvironmentsPageAction {
    Noop,
}

pub struct EnvironmentsPageView {
    current_page: EnvironmentsPage,
    pane_configuration: ModelHandle<PaneConfiguration>,
    focus_handle: Option<PaneFocusHandle>,
}

impl EnvironmentsPageView {
    pub fn new(ctx: &mut ViewContext<Self>) -> Self {
        Self {
            current_page: EnvironmentsPage::default(),
            pane_configuration: ctx.add_model(|_ctx| PaneConfiguration::new("Environments")),
            focus_handle: None,
        }
    }

    pub fn update_page(&mut self, page: EnvironmentsPage, ctx: &mut ViewContext<Self>) {
        self.current_page = page;
        ctx.notify();
    }

    pub fn current_page(&self) -> &EnvironmentsPage {
        &self.current_page
    }

    pub fn environment_setup_mode_selector_handle(
        &self,
    ) -> Option<&ViewHandle<EnvironmentSetupModeSelector>> {
        None
    }

    pub fn agent_assisted_environment_modal_handle(
        &self,
        _app: &AppContext,
    ) -> Option<&ViewHandle<UpdateEnvironmentForm>> {
        None
    }

    pub fn pane_configuration(&self) -> ModelHandle<PaneConfiguration> {
        self.pane_configuration.clone()
    }

    pub fn set_github_auth_redirect_target(
        &mut self,
        _target: GithubAuthRedirectTarget,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn focus(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.focus_self();
    }
}

impl Entity for EnvironmentsPageView {
    type Event = SettingsPageEvent;
}

impl TypedActionView for EnvironmentsPageView {
    type Action = EnvironmentsPageAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for EnvironmentsPageView {
    fn ui_name() -> &'static str {
        "EnvironmentsPage"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl SettingsPageMeta for EnvironmentsPageView {
    fn section() -> SettingsSection {
        SettingsSection::CloudEnvironments
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

impl BackingView for EnvironmentsPageView {
    type PaneHeaderOverflowMenuAction = EnvironmentsPageAction;
    type CustomAction = ();
    type AssociatedData = ();

    fn handle_pane_header_overflow_menu_action(
        &mut self,
        _action: &Self::PaneHeaderOverflowMenuAction,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    fn close(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(SettingsPageEvent::Pane(
            super::settings_page::PaneEventWrapper::Close,
        ));
    }

    fn focus_contents(&mut self, ctx: &mut ViewContext<Self>) {
        self.focus(ctx);
    }

    fn render_header_content(
        &self,
        _ctx: &HeaderRenderContext<'_>,
        _app: &AppContext,
    ) -> HeaderContent {
        HeaderContent::simple("Environments")
    }

    fn set_focus_handle(&mut self, focus_handle: PaneFocusHandle, _ctx: &mut ViewContext<Self>) {
        self.focus_handle = Some(focus_handle);
    }
}

impl From<ViewHandle<EnvironmentsPageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<EnvironmentsPageView>) -> Self {
        SettingsPageViewHandle::CloudEnvironments(view_handle)
    }
}
