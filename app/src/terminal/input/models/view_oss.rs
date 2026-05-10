use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, EntityId, ModelHandle, View, ViewContext};

use crate::ai::blocklist::agent_view::AgentViewController;
use crate::ai::blocklist::block::cli_controller::CLISubagentController;
use crate::ai::llms::LLMId;
use crate::terminal::input::buffer_model::InputBufferModel;
use crate::terminal::input::inline_menu::InlineMenuPositioner;
use crate::terminal::input::suggestions_mode_model::InputSuggestionsModeModel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlineModelSelectorTab {
    BaseAgent,
    FullTerminalUse,
}

#[derive(Debug, Clone)]
pub enum InlineModelSelectorEvent {
    SelectedModel {
        id: LLMId,
        selected_tab: InlineModelSelectorTab,
        set_as_default: bool,
    },
    Dismissed,
}

pub struct InlineModelSelectorView {
    active_tab: InlineModelSelectorTab,
    filter_results_by_input: bool,
    _terminal_view_id: EntityId,
}

impl InlineModelSelectorView {
    pub fn new(
        terminal_view_id: EntityId,
        _suggestions_mode_model: ModelHandle<InputSuggestionsModeModel>,
        _agent_view_controller: ModelHandle<AgentViewController>,
        _input_buffer_model: &ModelHandle<InputBufferModel>,
        _cli_subagent_controller: ModelHandle<CLISubagentController>,
        _positioner: &ModelHandle<InlineMenuPositioner>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            active_tab: InlineModelSelectorTab::BaseAgent,
            filter_results_by_input: true,
            _terminal_view_id: terminal_view_id,
        }
    }

    pub fn set_filter_results_by_input(&mut self, value: bool) {
        self.filter_results_by_input = value;
    }

    pub fn filter_results_by_input(&self) -> bool {
        self.filter_results_by_input
    }

    pub fn set_active_tab(&mut self, tab: InlineModelSelectorTab, ctx: &mut ViewContext<Self>) {
        self.active_tab = tab;
        ctx.notify();
    }

    pub fn select_up(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn select_down(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn select_next_tab(&mut self, _ctx: &mut ViewContext<Self>) -> bool {
        false
    }

    pub fn accept_selected_item(&mut self, _set_as_default: bool, ctx: &mut ViewContext<Self>) {
        ctx.emit(InlineModelSelectorEvent::Dismissed);
    }
}

impl View for InlineModelSelectorView {
    fn ui_name() -> &'static str {
        "InlineModelSelectorView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl Entity for InlineModelSelectorView {
    type Event = InlineModelSelectorEvent;
}
