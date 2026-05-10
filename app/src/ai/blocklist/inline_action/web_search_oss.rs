use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, TypedActionView, View, ViewContext};

use crate::ai::agent::WebSearchStatus;

pub enum WebSearchViewEvent {}

#[derive(Clone)]
pub enum WebSearchViewAction {
    ToggleExpanded,
}

impl std::fmt::Debug for WebSearchViewAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SearchAction")
    }
}

pub struct WebSearchView {
    pub status: WebSearchStatus,
}

impl WebSearchView {
    pub fn new(query: String) -> Self {
        Self {
            status: WebSearchStatus::Searching {
                query: if query.is_empty() { None } else { Some(query) },
            },
        }
    }

    pub fn set_status(&mut self, status: &WebSearchStatus) {
        self.status = status.clone();
    }
}

impl Entity for WebSearchView {
    type Event = WebSearchViewEvent;
}

impl TypedActionView for WebSearchView {
    type Action = WebSearchViewAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for WebSearchView {
    fn ui_name() -> &'static str {
        "SearchView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
