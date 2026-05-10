use std::{collections::HashMap, sync::Arc};

use warpui::{elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext};

use crate::{
    ai::agent::SuggestedAgentModeWorkflow,
    server::ids::SyncId,
    workflows::{WorkflowSelectionSource, WorkflowSource, WorkflowType},
};

#[derive(Debug, Clone, Default)]
pub struct SuggestedAgentModeWorkflowModal;

#[derive(Debug, Clone)]
pub struct SuggestedAgentModeWorkflowAndId {
    pub workflow: SuggestedAgentModeWorkflow,
    pub sync_id: SyncId,
}

#[derive(Debug, Clone)]
pub enum SuggestedAgentModeWorkflowModalAction {
    Cancel,
}

#[derive(Debug, Clone)]
pub enum SuggestedAgentModeWorkflowModalEvent {
    Close,
    WorkflowCreated,
    RunWorkflow {
        workflow: Arc<WorkflowType>,
        source: Box<WorkflowSource>,
        argument_override: Option<HashMap<String, String>>,
        workflow_selection_source: WorkflowSelectionSource,
    },
}

pub fn init(_app: &mut AppContext) {}

impl SuggestedAgentModeWorkflowModal {
    pub fn close(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(SuggestedAgentModeWorkflowModalEvent::Close);
    }

    pub fn open_workflow(
        &mut self,
        _workflow_and_id: &SuggestedAgentModeWorkflowAndId,
        ctx: &mut ViewContext<Self>,
    ) {
        self.close(ctx);
    }
}

impl Entity for SuggestedAgentModeWorkflowModal {
    type Event = SuggestedAgentModeWorkflowModalEvent;
}

impl View for SuggestedAgentModeWorkflowModal {
    fn ui_name() -> &'static str {
        "OSSDisabledSuggestedWorkflowModal"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for SuggestedAgentModeWorkflowModal {
    type Action = SuggestedAgentModeWorkflowModalAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            SuggestedAgentModeWorkflowModalAction::Cancel => self.close(ctx),
        }
    }
}
