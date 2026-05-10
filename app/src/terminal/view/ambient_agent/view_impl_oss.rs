use warp_terminal::model::BlockId;
use warpui::prelude::{Empty, Vector2F};
use warpui::{Element, ModelHandle, ViewContext, ViewHandle};

use super::{AmbientAgentViewModel, AmbientAgentViewModelEvent};
use crate::pane_group::TerminalViewResources;
use crate::terminal::view::TerminalView;

impl TerminalView {
    pub(in crate::terminal::view) fn show_out_of_credits_modal(
        &self,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(in crate::terminal::view) fn handle_ambient_agent_event(
        &mut self,
        _event: &AmbientAgentViewModelEvent,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(in crate::terminal::view) fn maybe_insert_setup_command_blocks(
        &mut self,
        _block_id: &BlockId,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(in crate::terminal::view) fn enter_cloud_agent_view(
        &mut self,
        _initial_prompt: Option<String>,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(in crate::terminal::view) fn enter_cloud_mode_from_session(
        &mut self,
        _initial_prompt: Option<String>,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    #[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
    pub(crate) fn start_local_to_cloud_handoff_pane(
        &mut self,
        _ctx: &mut ViewContext<Self>,
    ) -> Option<(ViewHandle<TerminalView>, ModelHandle<AmbientAgentViewModel>)> {
        None
    }

    pub(in crate::terminal::view) fn render_ambient_agent_progress(
        &self,
        _appearance: &warp_core::ui::appearance::Appearance,
        _app: &warpui::AppContext,
    ) -> Box<dyn Element> {
        Empty::new().finish()
    }

    pub(in crate::terminal::view) fn handle_first_time_cloud_agent_setup_event(
        &mut self,
        event: &super::FirstTimeCloudAgentSetupViewEvent,
        ctx: &mut ViewContext<Self>,
    ) {
        if matches!(event, super::FirstTimeCloudAgentSetupViewEvent::Cancelled) {
            self.exit_agent_view(ctx);
        }
    }

    pub(in crate::terminal::view) fn fetch_and_update_conversation_details_panel(
        &mut self,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(in crate::terminal::view) fn refresh_conversation_details_panel_if_open(
        &mut self,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(in crate::terminal::view) fn maybe_auto_open_conversation_details_panel(
        &mut self,
        _ctx: &mut ViewContext<Self>,
    ) {
    }
}

pub fn create_cloud_mode_view(
    _resources: TerminalViewResources,
    _view_bounds_size: Vector2F,
    _window_id: warpui::WindowId,
    _ctx: &mut warpui::AppContext,
) -> (
    ViewHandle<TerminalView>,
    ModelHandle<Box<dyn crate::terminal::TerminalManager>>,
) {
    unreachable!("cloud mode is disabled in OSS builds")
}
