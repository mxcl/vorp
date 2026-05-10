use warpui::{elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext};

use crate::tab_configs::session_config::SessionConfigSelection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoaOnboardingStep {
    WelcomeBanner,
    VerticalTabsCallout,
    AgentInboxCallout,
    TabConfig,
}

pub fn init(_app: &mut warpui::AppContext) {}

#[derive(Clone, Debug)]
pub enum HoaOnboardingAction {
    Dismiss,
}

pub enum HoaOnboardingFlowEvent {
    Completed(Option<SessionConfigSelection>),
    Dismissed,
    StepChanged,
    TabLayoutToggled,
}

pub struct HoaOnboardingFlow;

impl HoaOnboardingFlow {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn step(&self) -> HoaOnboardingStep {
        HoaOnboardingStep::WelcomeBanner
    }
}

impl Entity for HoaOnboardingFlow {
    type Event = HoaOnboardingFlowEvent;
}

impl View for HoaOnboardingFlow {
    fn ui_name() -> &'static str {
        "HoaOnboardingFlow"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for HoaOnboardingFlow {
    type Action = HoaOnboardingAction;

    fn handle_action(&mut self, _action: &Self::Action, ctx: &mut ViewContext<Self>) {
        ctx.emit(HoaOnboardingFlowEvent::Dismissed);
    }
}
