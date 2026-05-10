use std::sync::Arc;

use warpui::{Entity, ModelContext, SingletonEntity, WindowId};

use crate::{
    cloud_object::Owner, drive::OpenWarpDriveObjectSettings, pane_group::NotebookPane,
    server::ids::SyncId, workspace::PaneViewLocator,
};

/// Source for a new notebook pane.
#[derive(Debug, Clone)]
pub enum NotebookSource {
    Existing(SyncId),
    New {
        title: Option<String>,
        owner: Owner,
        initial_folder_id: Option<SyncId>,
    },
}

pub struct NotebookManager;

impl NotebookManager {
    pub fn new(
        _cached_notebooks: Vec<super::CloudNotebook>,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self
    }

    pub fn find_pane(&self, _source: &NotebookSource) -> Option<(WindowId, PaneViewLocator)> {
        None
    }

    pub fn notebook_raw_text(&self, _notebook_id: SyncId) -> Option<&str> {
        None
    }

    pub fn notebook_raw_text_shared(&self, _notebook_id: SyncId) -> Option<Arc<str>> {
        None
    }

    pub fn create_pane(
        &mut self,
        _source: &NotebookSource,
        _settings: &OpenWarpDriveObjectSettings,
        _window_id: WindowId,
        _ctx: &mut ModelContext<Self>,
    ) -> NotebookPane {
        panic!("disabled")
    }

    pub fn register_pane(
        &mut self,
        _pane: &NotebookPane,
        _pane_group_id: warpui::EntityId,
        _window_id: WindowId,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn deregister_pane(&mut self, _pane: &NotebookPane, _ctx: &mut ModelContext<Self>) {}

    pub(super) fn swap_notebook(&mut self, _old_id: SyncId, _new_id: SyncId) {}

    pub fn close_notebooks(&self, _ctx: &mut ModelContext<Self>) {}

    pub fn reset(&mut self) {}
}

impl Entity for NotebookManager {
    type Event = ();
}

impl SingletonEntity for NotebookManager {}
