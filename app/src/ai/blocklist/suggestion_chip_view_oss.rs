use pathfinder_color::ColorU;
use warp_core::ui::{appearance::Appearance, theme::Fill};
use warpui::{elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext};

use crate::{
    ai::{
        agent::{SuggestedAgentModeWorkflow, SuggestedLoggingId, SuggestedRule},
        blocklist::{
            suggested_agent_mode_workflow_modal::SuggestedAgentModeWorkflowAndId,
            suggested_rule_modal::SuggestedRuleAndId,
        },
    },
    server::ids::SyncId,
    ui_components::{blended_colors, icons::Icon},
    view_components::action_button::ActionButtonTheme,
};

pub struct SuggestionDismissButtonTheme;

impl ActionButtonTheme for SuggestionDismissButtonTheme {
    fn background(&self, hovered: bool, appearance: &Appearance) -> Option<Fill> {
        if hovered {
            Some(blended_colors::fg_overlay_2(appearance.theme()))
        } else {
            None
        }
    }

    fn text_color(
        &self,
        _hovered: bool,
        _background: Option<Fill>,
        appearance: &Appearance,
    ) -> ColorU {
        appearance
            .theme()
            .sub_text_color(appearance.theme().background())
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum SuggestedChipViewEvent {
    ShowSuggestedRuleDialog {
        rule_and_id: SuggestedRuleAndId,
    },
    OpenAIFactCollection {
        sync_id: Option<SyncId>,
    },
    OpenWorkflow {
        sync_id: SyncId,
    },
    ShowSuggestedAgentModeWorkflowModal {
        workflow_and_id: SuggestedAgentModeWorkflowAndId,
    },
}

#[derive(Debug, Clone)]
pub enum SuggestedViewAction {
    ChipClicked,
}

#[derive(Clone)]
pub struct SuggestionChipView {
    logging_id: SuggestedLoggingId,
}

impl SuggestionChipView {
    pub fn new_rule_chip(rule: SuggestedRule, _ctx: &mut ViewContext<Self>) -> Self {
        Self {
            logging_id: rule.logging_id,
        }
    }

    pub fn new_agent_mode_workflow_chip(
        workflow: SuggestedAgentModeWorkflow,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            logging_id: workflow.logging_id,
        }
    }

    pub fn logging_id(&self) -> SuggestedLoggingId {
        self.logging_id.clone()
    }
}

impl Entity for SuggestionChipView {
    type Event = SuggestedChipViewEvent;
}

impl View for SuggestionChipView {
    fn ui_name() -> &'static str {
        "OSSDisabledSuggestionChipView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for SuggestionChipView {
    type Action = SuggestedViewAction;

    fn handle_action(&mut self, action: &SuggestedViewAction, _ctx: &mut ViewContext<Self>) {
        match action {
            SuggestedViewAction::ChipClicked => {}
        }
    }
}
