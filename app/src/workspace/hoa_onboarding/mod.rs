#[cfg(not(feature = "oss_release"))]
mod hoa_onboarding_flow;
#[cfg(feature = "oss_release")]
#[path = "hoa_onboarding_flow_oss.rs"]
mod hoa_onboarding_flow;
#[cfg(not(feature = "oss_release"))]
mod tab_config_step;
#[cfg(not(feature = "oss_release"))]
mod welcome_banner;

pub use hoa_onboarding_flow::{init, HoaOnboardingFlow, HoaOnboardingFlowEvent, HoaOnboardingStep};

use warpui::AppContext;

use warp_core::user_preferences::GetUserPreferences;

const HAS_COMPLETED_HOA_ONBOARDING_KEY: &str = "HasCompletedHOAOnboarding";

pub fn has_completed_hoa_onboarding(ctx: &AppContext) -> bool {
    #[cfg(feature = "oss_release")]
    {
        let _ = ctx;
        return true;
    }

    #[cfg(not(feature = "oss_release"))]
    ctx.private_user_preferences()
        .read_value(HAS_COMPLETED_HOA_ONBOARDING_KEY)
        .unwrap_or_default()
        .and_then(|s| serde_json::from_str::<bool>(&s).ok())
        .unwrap_or(false)
}

pub fn mark_hoa_onboarding_completed(ctx: &AppContext) {
    #[cfg(feature = "oss_release")]
    {
        let _ = ctx;
        return;
    }

    #[cfg(not(feature = "oss_release"))]
    let _ = ctx.private_user_preferences().write_value(
        HAS_COMPLETED_HOA_ONBOARDING_KEY,
        serde_json::to_string(&true).expect("bool serializes to JSON"),
    );
}
