use crate::{ai::agent::SuggestedRule, server::ids::SyncId};
use warpui::{elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext};

pub fn init(_app: &mut AppContext) {}

#[derive(Debug, Clone)]
pub enum SuggestedRuleModalEvent {
    AddNewRule { rule: SuggestedRule },
    OpenRuleForEditing { rule: SuggestedRule },
    Close,
}

#[derive(Debug, Clone)]
pub struct SuggestedRuleAndId {
    pub rule: SuggestedRule,
    pub sync_id: SyncId,
}

pub struct SuggestedRuleModal {
    rule_and_id: Option<SuggestedRuleAndId>,
}

impl SuggestedRuleModal {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self { rule_and_id: None }
    }

    pub fn set_rule_and_id(
        &mut self,
        rule_and_id: &SuggestedRuleAndId,
        ctx: &mut ViewContext<Self>,
    ) {
        self.rule_and_id = Some(rule_and_id.clone());
        ctx.emit(SuggestedRuleModalEvent::Close);
    }
}

impl Entity for SuggestedRuleModal {
    type Event = SuggestedRuleModalEvent;
}

impl View for SuggestedRuleModal {
    fn ui_name() -> &'static str {
        "SuggestedRuleModal"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for SuggestedRuleModal {
    type Action = ();

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}
