use warpui::{Entity, ModelContext, SingletonEntity, WindowId};

pub struct OneTimeModalModel {
    target_window_id: Option<WindowId>,
}

impl OneTimeModalModel {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self {
            target_window_id: None,
        }
    }

    pub fn is_oz_launch_modal_open(&self) -> bool {
        false
    }

    pub fn target_window_id(&self) -> Option<WindowId> {
        self.target_window_id
    }

    pub fn mark_oz_launch_modal_dismissed(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn is_openwarp_launch_modal_open(&self) -> bool {
        false
    }

    pub fn mark_openwarp_launch_modal_dismissed(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn is_hoa_onboarding_open(&self) -> bool {
        false
    }

    pub fn mark_hoa_onboarding_dismissed(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn is_any_modal_open(&self) -> bool {
        false
    }

    #[cfg(debug_assertions)]
    pub fn force_open_oz_launch_modal(&mut self, _ctx: &mut ModelContext<Self>) {}

    #[cfg(debug_assertions)]
    pub fn force_open_openwarp_launch_modal(&mut self, _ctx: &mut ModelContext<Self>) {}

    #[cfg(debug_assertions)]
    pub fn force_open_build_plan_migration_modal(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn update_target_window_id(&mut self, window_id: WindowId, _ctx: &mut ModelContext<Self>) {
        self.target_window_id = Some(window_id);
    }

    pub fn is_build_plan_migration_modal_open(&self) -> bool {
        false
    }

    pub fn mark_build_plan_migration_modal_dismissed(&mut self, _ctx: &mut ModelContext<Self>) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OneTimeModalEvent {
    VisibilityChanged { is_open: bool },
}

impl Entity for OneTimeModalModel {
    type Event = OneTimeModalEvent;
}

impl SingletonEntity for OneTimeModalModel {}
