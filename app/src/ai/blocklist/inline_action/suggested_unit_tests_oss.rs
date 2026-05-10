use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, FocusContext, TypedActionView, View, ViewContext};

use crate::ai::agent::{AIAgentActionId, AIIdentifiers};

#[derive(Clone)]
pub enum SuggestedUnitTestsEvent {
    Accept,
    Cancel,
    Blur,
    OpenSettings,
}

impl std::fmt::Debug for SuggestedUnitTestsEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SuggestionEvent")
    }
}

#[derive(Clone)]
pub enum SuggestedUnitTestsAction {
    Accept,
    Cancel,
    ToggleSetting,
    OpenSettings,
}

impl std::fmt::Debug for SuggestedUnitTestsAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SuggestionAction")
    }
}

pub struct SuggestedUnitTestsView {
    identifiers: AIIdentifiers,
    action_id: AIAgentActionId,
    is_hidden: bool,
    is_keybindings_hidden: bool,
    query: String,
}

impl SuggestedUnitTestsView {
    pub fn new(
        identifiers: AIIdentifiers,
        action_id: AIAgentActionId,
        query: String,
        _title: String,
        _description: String,
        _should_show_speedbump: bool,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            identifiers,
            action_id,
            is_hidden: true,
            is_keybindings_hidden: true,
            query,
        }
    }

    pub fn identifiers(&self) -> &AIIdentifiers {
        &self.identifiers
    }

    pub fn action_id(&self) -> &AIAgentActionId {
        &self.action_id
    }

    pub fn is_hidden(&self) -> bool {
        self.is_hidden
    }

    pub fn is_keybindings_hidden(&self) -> bool {
        self.is_keybindings_hidden
    }

    pub fn query(&self) -> Option<String> {
        (!self.query.is_empty()).then(|| self.query.to_string())
    }

    pub fn set_is_hidden(&mut self, is_hidden: bool) {
        self.is_hidden = is_hidden;
    }

    pub fn hide_keybindings(&mut self, ctx: &mut ViewContext<Self>) {
        self.is_keybindings_hidden = true;
        ctx.notify();
    }
}

impl View for SuggestedUnitTestsView {
    fn ui_name() -> &'static str {
        "SuggestionView"
    }

    fn on_focus(&mut self, _focus_ctx: &FocusContext, ctx: &mut ViewContext<Self>) {
        ctx.emit(SuggestedUnitTestsEvent::Blur);
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl Entity for SuggestedUnitTestsView {
    type Event = SuggestedUnitTestsEvent;
}

impl TypedActionView for SuggestedUnitTestsView {
    type Action = SuggestedUnitTestsAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            SuggestedUnitTestsAction::Accept => ctx.emit(SuggestedUnitTestsEvent::Accept),
            SuggestedUnitTestsAction::Cancel => ctx.emit(SuggestedUnitTestsEvent::Cancel),
            SuggestedUnitTestsAction::ToggleSetting => {}
            SuggestedUnitTestsAction::OpenSettings => {
                ctx.emit(SuggestedUnitTestsEvent::OpenSettings)
            }
        }
    }
}
