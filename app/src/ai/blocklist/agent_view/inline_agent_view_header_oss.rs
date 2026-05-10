use std::sync::Arc;

use parking_lot::FairMutex;
use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, EntityId, ModelHandle, View, ViewContext};

use crate::ai::blocklist::BlocklistAIActionModel;
use crate::terminal::{model::session::Sessions, TerminalModel};

pub type InlineAgentViewHeader = HeaderView;

pub struct HeaderView;

impl HeaderView {
    pub fn new(
        _terminal_view_id: EntityId,
        _terminal_model: Arc<FairMutex<TerminalModel>>,
        _sessions_model: ModelHandle<Sessions>,
        _action_model: ModelHandle<BlocklistAIActionModel>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }
}

impl Entity for HeaderView {
    type Event = ();
}

impl View for HeaderView {
    fn ui_name() -> &'static str {
        "HeaderView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
