use std::sync::Arc;

use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, EntityId, ModelHandle, TypedActionView, View, ViewContext};

use crate::ai::blocklist::BlocklistAIContextModel;
use crate::ai::document::ai_document_model::{AIDocumentId, AIDocumentVersion};
use crate::terminal::input::MenuPositioningProvider;

pub type PlanAndTodoListView = ChipView;
pub type PlanAndTodoListAction = ChipAction;
pub type PlanAndTodoListEvent = ChipEvent;

pub struct ChipView;

pub enum ChipEvent {
    OpenAIDocument {
        document_id: AIDocumentId,
        document_version: AIDocumentVersion,
    },
}

#[derive(Clone, PartialEq, Eq)]
pub enum ChipAction {
    ToggleTodoPopup,
    OpenAIDocument {
        document_id: AIDocumentId,
        document_version: AIDocumentVersion,
    },
}

impl std::fmt::Debug for ChipAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Action")
    }
}

impl ChipView {
    pub fn new(
        _context_model: ModelHandle<BlocklistAIContextModel>,
        _menu_positioning_provider: Arc<dyn MenuPositioningProvider>,
        _terminal_view_id: EntityId,
        _is_in_agent_view: bool,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn should_render(&self, _app: &AppContext) -> bool {
        false
    }
}

impl Entity for ChipView {
    type Event = ChipEvent;
}

impl View for ChipView {
    fn ui_name() -> &'static str {
        "ChipView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for ChipView {
    type Action = ChipAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        if let ChipAction::OpenAIDocument {
            document_id,
            document_version,
        } = action
        {
            ctx.emit(ChipEvent::OpenAIDocument {
                document_id: *document_id,
                document_version: *document_version,
            });
        }
    }
}
