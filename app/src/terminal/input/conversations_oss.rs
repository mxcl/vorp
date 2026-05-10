//! OSS no-op inline conversation menu.

use warpui::elements::Empty;
use warpui::{AppContext, Element, Entity, ModelHandle, View, ViewContext};

use crate::ai::blocklist::agent_view::AgentViewController;
use crate::terminal::input::buffer_model::InputBufferModel;
use crate::terminal::input::inline_menu::InlineMenuPositioner;
use crate::terminal::input::suggestions_mode_model::InputSuggestionsModeModel;
use crate::terminal::model::session::active_session::ActiveSession;

#[derive(Debug, Clone)]
pub enum InlineConversationMenuEvent {
    Dismissed,
}

pub struct InlineConversationMenuView;

impl InlineConversationMenuView {
    pub fn new(
        _input_suggestions_model: ModelHandle<InputSuggestionsModeModel>,
        _agent_view_controller: ModelHandle<AgentViewController>,
        _input_buffer_model: &ModelHandle<InputBufferModel>,
        _positioner: &ModelHandle<InlineMenuPositioner>,
        _active_session: ModelHandle<ActiveSession>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn select_next_tab(&self, _ctx: &mut ViewContext<Self>) -> bool {
        false
    }

    pub fn select_up(&self, _ctx: &mut ViewContext<Self>) {}

    pub fn select_down(&self, _ctx: &mut ViewContext<Self>) {}

    pub fn accept_selected_item(&self, _ctx: &mut ViewContext<Self>) {}
}

impl View for InlineConversationMenuView {
    fn ui_name() -> &'static str {
        "InlineConversationMenuView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl Entity for InlineConversationMenuView {
    type Event = InlineConversationMenuEvent;
}
