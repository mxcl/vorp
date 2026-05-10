use warpui::{elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum CloudAgentCapacityModalVariant {
    #[default]
    ConcurrentLimit,
    OutOfCredits,
}

pub fn init(_app: &mut AppContext) {}

pub struct CloudAgentCapacityModal {
    variant: CloudAgentCapacityModalVariant,
}

impl CloudAgentCapacityModal {
    pub fn new() -> Self {
        Self {
            variant: CloudAgentCapacityModalVariant::default(),
        }
    }

    pub fn set_variant(&mut self, variant: CloudAgentCapacityModalVariant) {
        self.variant = variant;
    }
}

impl Entity for CloudAgentCapacityModal {
    type Event = CloudAgentCapacityModalEvent;
}

impl View for CloudAgentCapacityModal {
    fn ui_name() -> &'static str {
        "CloudAgentCapacityModal"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for CloudAgentCapacityModal {
    type Action = CloudAgentCapacityModalAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

#[derive(Copy, Clone, Debug)]
pub enum CloudAgentCapacityModalEvent {
    Close,
}

#[derive(Clone, Debug)]
pub enum CloudAgentCapacityModalAction {
    Close,
    Upgrade,
}
