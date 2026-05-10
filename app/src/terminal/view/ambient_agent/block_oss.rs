use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::FairMutex;
use warp_terminal::model::BlockId;
use warpui::prelude::{Container, Empty};
use warpui::{
    AppContext, Element, Entity, ModelHandle, SingletonEntity, TypedActionView, View, ViewContext,
    ViewHandle, WeakModelHandle,
};

use crate::ai::blocklist::agent_view::AgentViewController;
use crate::pane_group::pane::PaneStack;
use crate::terminal::model_events::ModelEventDispatcher;
use crate::terminal::{TerminalManager, TerminalModel, TerminalView};

use super::AmbientAgentViewModel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetupCommandGroupId(u64);

impl SetupCommandGroupId {
    fn initial() -> Self {
        Self(0)
    }
}

#[derive(Debug, Clone)]
pub struct SetupCommandState {
    did_execute_a_setup_command: bool,
    current_group_id: SetupCommandGroupId,
    next_group_id: u64,
    expanded_groups: HashMap<SetupCommandGroupId, bool>,
    running_group_id: Option<SetupCommandGroupId>,
}

impl Default for SetupCommandState {
    fn default() -> Self {
        let current_group_id = SetupCommandGroupId::initial();
        let mut expanded_groups = HashMap::new();
        expanded_groups.insert(current_group_id, true);
        Self {
            did_execute_a_setup_command: false,
            current_group_id,
            next_group_id: 1,
            expanded_groups,
            running_group_id: Some(current_group_id),
        }
    }
}

impl SetupCommandState {
    pub fn current_group_id(&self) -> SetupCommandGroupId {
        self.current_group_id
    }

    pub fn did_execute_a_setup_command(&self) -> bool {
        self.did_execute_a_setup_command
    }

    pub fn set_did_execute_a_setup_command(&mut self, value: bool) {
        self.did_execute_a_setup_command = value;
    }

    pub fn should_expand(&self, group_id: SetupCommandGroupId) -> bool {
        self.expanded_groups.get(&group_id).copied().unwrap_or(true)
    }

    pub fn set_should_expand(&mut self, group_id: SetupCommandGroupId, value: bool) {
        self.expanded_groups.insert(group_id, value);
    }

    pub fn is_running(&self, group_id: SetupCommandGroupId) -> bool {
        self.running_group_id == Some(group_id)
    }

    pub fn start_new_group(&mut self) -> SetupCommandGroupId {
        let group_id = SetupCommandGroupId(self.next_group_id);
        self.next_group_id += 1;
        self.current_group_id = group_id;
        self.did_execute_a_setup_command = false;
        self.expanded_groups.insert(group_id, true);
        self.running_group_id = Some(group_id);
        group_id
    }

    pub fn finish_group(&mut self, group_id: SetupCommandGroupId) {
        if self.running_group_id == Some(group_id) {
            self.running_group_id = None;
        }
    }
}

pub struct AmbientAgentEntryBlock;

impl AmbientAgentEntryBlock {
    pub fn new(
        _terminal_view: ViewHandle<TerminalView>,
        _terminal_manager: ModelHandle<Box<dyn TerminalManager>>,
        _pane_stack: WeakModelHandle<PaneStack<TerminalView>>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }
}

impl Entity for AmbientAgentEntryBlock {
    type Event = ();
}

impl View for AmbientAgentEntryBlock {
    fn ui_name() -> &'static str {
        "AmbientAgentEntryBlock"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

#[derive(Debug, Clone)]
pub enum AmbientAgentEntryBlockAction {
    OpenAmbientAgent,
}

impl TypedActionView for AmbientAgentEntryBlock {
    type Action = AmbientAgentEntryBlockAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

pub struct CloudModeSetupCommandBlock;

impl CloudModeSetupCommandBlock {
    pub fn new(
        _group_id: SetupCommandGroupId,
        _block_id: BlockId,
        _ambient_agent_view_model: ModelHandle<AmbientAgentViewModel>,
        _model_events: &ModelHandle<ModelEventDispatcher>,
        _terminal_model: Arc<FairMutex<TerminalModel>>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub enum CloudModeSetupCommandBlockEvent {
    ToggleBlockVisibility(BlockId),
}

impl Entity for CloudModeSetupCommandBlock {
    type Event = CloudModeSetupCommandBlockEvent;
}

impl View for CloudModeSetupCommandBlock {
    fn ui_name() -> &'static str {
        "CloudModeSetupCommandBlock"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

#[derive(Debug, Clone)]
pub enum CloudModeSetupCommandBlockAction {
    ToggleBlockVisibility,
}

impl TypedActionView for CloudModeSetupCommandBlock {
    type Action = CloudModeSetupCommandBlockAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

pub struct CloudModeSetupTextBlock;

impl CloudModeSetupTextBlock {
    pub fn new(
        _group_id: SetupCommandGroupId,
        _ambient_agent_view_model: ModelHandle<AmbientAgentViewModel>,
        _agent_view_controller: ModelHandle<AgentViewController>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }
}

impl Entity for CloudModeSetupTextBlock {
    type Event = ();
}

impl View for CloudModeSetupTextBlock {
    fn ui_name() -> &'static str {
        "CloudModeSetupTextBlock"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

#[derive(Debug, Clone)]
pub enum CloudModeSetupTextBlockAction {
    ToggleSetupCommandVisibility,
}

impl TypedActionView for CloudModeSetupTextBlock {
    type Action = CloudModeSetupTextBlockAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

pub(super) fn cloud_mode_setup_row_spacing(
    element: Box<dyn Element>,
    _ambient_agent_view_model: &ModelHandle<AmbientAgentViewModel>,
    _app: &AppContext,
) -> Container {
    Container::new(element)
}

pub(super) fn cloud_mode_setup_text_row_spacing(
    element: Box<dyn Element>,
    _ambient_agent_view_model: &ModelHandle<AmbientAgentViewModel>,
    _app: &AppContext,
) -> Container {
    Container::new(element)
}
