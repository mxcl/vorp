use crate::{ai::facts::AIFactView, pane_group::AIFactPane, PaneViewLocator};
use warpui::{Entity, EntityId, ModelContext, SingletonEntity, ViewHandle, WindowId};

#[derive(Default)]
pub struct AIFactManager;

impl AIFactManager {
    pub fn new() -> Self {
        Self
    }

    pub fn ai_fact_view(&self, _window_id: WindowId) -> ViewHandle<AIFactView> {
        panic!("AI Rules panes are not available in this build")
    }

    pub fn register_view(&mut self, _window_id: WindowId, _view: ViewHandle<AIFactView>) {}

    pub fn find_pane(&self, _window_id: WindowId) -> Option<PaneViewLocator> {
        None
    }

    pub fn register_pane(
        &mut self,
        _pane: &AIFactPane,
        _pane_group_id: EntityId,
        _window_id: WindowId,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn deregister_pane(&mut self, _window_id: &WindowId, _ctx: &mut ModelContext<Self>) {}
}

impl Entity for AIFactManager {
    type Event = ();
}

impl SingletonEntity for AIFactManager {}
