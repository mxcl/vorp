use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, EntityId, ModelHandle, View, ViewContext};

use crate::ai::blocklist::agent_view::AgentViewController;
use crate::ai::execution_profiles::profiles::ClientProfileId;
use crate::terminal::input::buffer_model::InputBufferModel;
use crate::terminal::input::inline_menu::InlineMenuPositioner;
use crate::terminal::input::suggestions_mode_model::InputSuggestionsModeModel;

#[derive(Debug, Clone)]
pub enum InlineProfileSelectorEvent {
    SelectedProfile { profile_id: ClientProfileId },
    ManageProfiles,
    Dismissed,
}

pub struct InlineProfileSelectorView {
    _terminal_view_id: EntityId,
}

impl InlineProfileSelectorView {
    pub fn new(
        terminal_view_id: EntityId,
        _suggestions_mode_model: ModelHandle<InputSuggestionsModeModel>,
        _agent_view_controller: ModelHandle<AgentViewController>,
        _input_buffer_model: &ModelHandle<InputBufferModel>,
        _positioner: &ModelHandle<InlineMenuPositioner>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            _terminal_view_id: terminal_view_id,
        }
    }

    pub fn select_up(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn select_down(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn accept_selected_item(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(InlineProfileSelectorEvent::Dismissed);
    }
}

impl View for InlineProfileSelectorView {
    fn ui_name() -> &'static str {
        "InlineProfileSelectorView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl Entity for InlineProfileSelectorView {
    type Event = InlineProfileSelectorEvent;
}
