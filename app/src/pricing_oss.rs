use warpui::{Entity, ModelContext, SingletonEntity};

/// OSS builds do not use server-side pricing data. This compatibility model
/// keeps metadata update call sites intact without linking GraphQL billing types.
#[derive(Debug, Default)]
pub struct PricingInfoModel;

impl PricingInfoModel {
    pub fn new() -> Self {
        Self
    }

    pub fn update_pricing_info<T>(&mut self, _pricing_info: T, ctx: &mut ModelContext<Self>) {
        ctx.emit(PricingInfoModelEvent::PricingInfoUpdated);
    }

    #[allow(dead_code)]
    pub fn plan_pricing<T>(&self, _plan: &T) -> Option<()> {
        None
    }

    #[allow(dead_code)]
    pub fn overage_cost_dollars(&self) -> Option<f64> {
        None
    }

    #[allow(dead_code)]
    pub fn monthly_plan_cost_dollars<T>(&self, _plan: &T) -> Option<f64> {
        None
    }

    #[allow(dead_code)]
    pub fn addon_credits_options<T>(&self) -> Option<&[T]> {
        None
    }
}

#[derive(Debug, Clone)]
pub enum PricingInfoModelEvent {
    PricingInfoUpdated,
}

impl Entity for PricingInfoModel {
    type Event = PricingInfoModelEvent;
}

impl SingletonEntity for PricingInfoModel {}
