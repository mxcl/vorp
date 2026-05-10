use warpui::{
    elements::Empty, text_layout::ClipConfig, AppContext, Element, Entity, ModelHandle,
    TypedActionView, View, ViewContext,
};

use crate::pane_group::{
    focus_state::PaneFocusHandle,
    pane::view::{self, HeaderContent, StandardHeader, StandardHeaderOptions},
    BackingView, PaneConfiguration, PaneEvent,
};

pub const NETWORK_LOG_HEADER_TEXT: &str = "Network log";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkLogViewEvent {
    Pane(PaneEvent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkLogViewAction {}

pub struct NetworkLogView {
    pane_configuration: ModelHandle<PaneConfiguration>,
    focus_handle: Option<PaneFocusHandle>,
}

impl NetworkLogView {
    pub fn new(ctx: &mut ViewContext<Self>) -> Self {
        let pane_configuration =
            ctx.add_model(|_ctx| PaneConfiguration::new(NETWORK_LOG_HEADER_TEXT));
        Self {
            pane_configuration,
            focus_handle: None,
        }
    }

    pub fn pane_configuration(&self) -> ModelHandle<PaneConfiguration> {
        self.pane_configuration.clone()
    }

    pub fn focus(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.focus_self();
    }

    pub fn reload_snapshot(&self, _ctx: &mut ViewContext<Self>) {}

    pub fn set_focus_handle(
        &mut self,
        focus_handle: PaneFocusHandle,
        _ctx: &mut ViewContext<Self>,
    ) {
        self.focus_handle = Some(focus_handle);
    }
}

impl Entity for NetworkLogView {
    type Event = NetworkLogViewEvent;
}

impl View for NetworkLogView {
    fn ui_name() -> &'static str {
        "NetworkLogView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for NetworkLogView {
    type Action = NetworkLogViewAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl BackingView for NetworkLogView {
    type PaneHeaderOverflowMenuAction = NetworkLogViewAction;
    type CustomAction = NetworkLogViewAction;
    type AssociatedData = ();

    fn handle_pane_header_overflow_menu_action(
        &mut self,
        _action: &Self::PaneHeaderOverflowMenuAction,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    fn close(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(NetworkLogViewEvent::Pane(PaneEvent::Close));
    }

    fn focus_contents(&mut self, ctx: &mut ViewContext<Self>) {
        self.focus(ctx);
    }

    fn render_header_content(
        &self,
        _ctx: &view::HeaderRenderContext<'_>,
        _app: &AppContext,
    ) -> HeaderContent {
        HeaderContent::Standard(StandardHeader {
            title: NETWORK_LOG_HEADER_TEXT.to_string(),
            title_secondary: None,
            title_style: None,
            title_clip_config: ClipConfig::start(),
            title_max_width: None,
            left_of_title: None,
            right_of_title: None,
            left_of_overflow: None,
            options: StandardHeaderOptions::default(),
        })
    }

    fn set_focus_handle(&mut self, focus_handle: PaneFocusHandle, _ctx: &mut ViewContext<Self>) {
        self.focus_handle = Some(focus_handle);
    }
}
