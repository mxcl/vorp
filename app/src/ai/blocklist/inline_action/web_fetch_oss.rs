use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, TypedActionView, View, ViewContext};

use crate::ai::agent::WebFetchStatus;

pub enum WebFetchViewEvent {}

#[derive(Clone)]
pub enum WebFetchViewAction {
    ToggleExpanded,
}

impl std::fmt::Debug for WebFetchViewAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("FetchAction")
    }
}

pub struct WebFetchView {
    pub status: WebFetchStatus,
}

impl WebFetchView {
    pub fn new(urls: Vec<String>) -> Self {
        Self {
            status: WebFetchStatus::Fetching { urls },
        }
    }

    pub fn set_status(&mut self, status: &WebFetchStatus) {
        self.status = status.clone();
    }
}

impl Entity for WebFetchView {
    type Event = WebFetchViewEvent;
}

impl TypedActionView for WebFetchView {
    type Action = WebFetchViewAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for WebFetchView {
    fn ui_name() -> &'static str {
        "FetchView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
