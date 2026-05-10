use crate::{
    workspaces::{user_workspaces::UserWorkspaces, workspace::UgcCollectionEnablementSetting},
    FeatureFlag,
};
use warpui::{elements::Empty, AppContext, Element, Entity, SingletonEntity, View, ViewContext};

#[derive(Default, Debug, Clone)]
pub struct TelemetryBanner;

impl TelemetryBanner {
    pub fn new(_is_onboarded: bool, _ctx: &mut ViewContext<Self>) -> Self {
        Self
    }
}

impl View for TelemetryBanner {
    fn ui_name() -> &'static str {
        "TelemetryBanner"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl Entity for TelemetryBanner {
    type Event = ();
}

pub fn should_collect_ai_ugc_telemetry(app: &AppContext, is_telemetry_enabled: bool) -> bool {
    match UserWorkspaces::as_ref(app).get_ugc_collection_enablement_setting() {
        UgcCollectionEnablementSetting::Disable => false,
        UgcCollectionEnablementSetting::Enable => true,
        UgcCollectionEnablementSetting::RespectUserSetting => {
            (FeatureFlag::GlobalAIAnalyticsCollection.is_enabled() && is_telemetry_enabled)
                || FeatureFlag::AgentModeAnalytics.is_enabled()
        }
    }
}
