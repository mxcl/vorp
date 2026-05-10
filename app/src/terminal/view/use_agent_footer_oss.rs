use std::sync::Arc;

use parking_lot::FairMutex;
use warpui::{
    elements::Empty, AppContext, Element, Entity, EntityId, ModelHandle, TypedActionView, View,
    ViewContext,
};

use crate::{
    ai::blocklist::agent_view::agent_input_footer::AgentInputFooter,
    terminal::{
        cli_agent_sessions::{CLIAgentInputEntrypoint, CLIAgentRichInputCloseReason},
        model_events::ModelEventDispatcher,
        CLIAgent, TerminalModel,
    },
};

use super::{block_banner::WarpificationMode, TerminalView};

pub struct UseAgentToolbar;

impl UseAgentToolbar {
    pub(crate) fn new(
        _terminal_view_id: EntityId,
        _terminal_model: Arc<FairMutex<TerminalModel>>,
        _model_event_dispatcher: &ModelHandle<ModelEventDispatcher>,
        _agent_input_footer: warpui::ViewHandle<AgentInputFooter>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub(in crate::terminal) fn notify_and_notify_children(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.notify();
    }

    pub(in crate::terminal) fn set_warpify_mode(
        &mut self,
        _mode: WarpificationMode,
        ctx: &mut ViewContext<Self>,
    ) {
        ctx.notify();
    }

    pub(in crate::terminal) fn clear_warpify_mode(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.notify();
    }

    pub(in crate::terminal) fn warpify_mode(&self, _app: &AppContext) -> Option<WarpificationMode> {
        None
    }

    #[cfg(feature = "voice_input")]
    pub fn has_cli_agent(&self, _app: &AppContext) -> bool {
        false
    }
}

impl Entity for UseAgentToolbar {
    type Event = ();
}

impl View for UseAgentToolbar {
    fn ui_name() -> &'static str {
        "UseAgentToolbar"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for UseAgentToolbar {
    type Action = ();
}

impl TerminalView {
    pub(super) fn register_subscriptions_for_use_agent_footer(&self, _ctx: &mut ViewContext<Self>) {
    }

    pub(super) fn has_active_cli_agent_input_session(&self, _app: &AppContext) -> bool {
        false
    }

    pub(super) fn should_render_use_agent_footer(
        &self,
        _model: &TerminalModel,
        _app: &AppContext,
    ) -> bool {
        false
    }

    pub(super) fn detect_cli_agent_from_model(
        &self,
        _model: &TerminalModel,
        _ctx: &AppContext,
    ) -> Option<(CLIAgent, Option<String>)> {
        None
    }

    pub(super) fn tag_in_agent_for_user_long_running_command(
        &mut self,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn tag_out_agent_for_user_long_running_command(
        &mut self,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn maybe_show_use_agent_footer_in_blocklist(&mut self, ctx: &mut ViewContext<Self>) {
        self.hide_use_agent_footer_in_blocklist(ctx);
    }

    pub(super) fn hide_use_agent_footer_in_blocklist(&mut self, ctx: &mut ViewContext<Self>) {
        self.model
            .lock()
            .block_list_mut()
            .remove_rich_content(self.use_agent_footer.id());
        ctx.notify();
    }

    pub(in crate::terminal) fn close_cli_agent_rich_input(
        &mut self,
        _reason: CLIAgentRichInputCloseReason,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(in crate::terminal) fn close_cli_agent_rich_input_and_disable_auto_toggle(
        &mut self,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn submit_cli_agent_rich_input(
        &mut self,
        _text: String,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    #[cfg(feature = "local_tty")]
    pub(crate) fn submit_text_to_cli_agent_pty(
        &mut self,
        _text: String,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn paste_dropped_images_to_cli_agent(
        &mut self,
        _image_filepaths: Vec<String>,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(in crate::terminal) fn open_cli_agent_rich_input(
        &mut self,
        _entrypoint: CLIAgentInputEntrypoint,
        _ctx: &mut ViewContext<Self>,
    ) {
    }
}
