use serde::{Deserialize, Serialize};
use warp_core::ui::{icons::Icon, theme::WarpTheme};
use warpui::{
    elements::{Element, Empty},
    AppContext, Entity, SingletonEntity, TypedActionView, View, ViewContext,
};

pub const AI_FEATURES: &[&str] = &[];
pub const WARP_DRIVE_FEATURES: &[&str] = &[];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OnboardingIntention {
    Terminal,
    AgentDrivenDevelopment,
}

impl std::fmt::Display for OnboardingIntention {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AgentDrivenDevelopment => write!(f, "agent_driven"),
            Self::Terminal => write!(f, "terminal"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LLMId(String);

impl LLMId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for LLMId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for LLMId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<LLMId> for String {
    fn from(value: LLMId) -> Self {
        value.0
    }
}

impl std::fmt::Display for LLMId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SessionDefault {
    #[default]
    Agent,
    Terminal,
}

impl std::fmt::Display for SessionDefault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Agent => write!(f, "agent"),
            Self::Terminal => write!(f, "terminal"),
        }
    }
}

pub mod slides {
    use super::{Icon, LLMId, SessionDefault};

    #[derive(Clone, Debug)]
    pub struct OnboardingModelInfo {
        pub id: LLMId,
        pub title: String,
        pub icon: Icon,
        pub requires_upgrade: bool,
        pub is_default: bool,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum AgentAutonomy {
        Full,
        #[default]
        Partial,
        None,
    }

    impl std::fmt::Display for AgentAutonomy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Full => write!(f, "full"),
                Self::Partial => write!(f, "partial"),
                Self::None => write!(f, "none"),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct AgentDevelopmentSettings {
        pub selected_model_id: LLMId,
        pub autonomy: Option<AgentAutonomy>,
        pub cli_agent_toolbar_enabled: bool,
        pub session_default: SessionDefault,
        pub disable_oz: bool,
        pub show_agent_notifications: bool,
    }

    impl AgentDevelopmentSettings {
        pub fn new(default_model_id: LLMId) -> Self {
            Self {
                selected_model_id: default_model_id,
                autonomy: Some(AgentAutonomy::default()),
                cli_agent_toolbar_enabled: true,
                session_default: SessionDefault::Agent,
                disable_oz: false,
                show_agent_notifications: true,
            }
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub enum ProjectOnboardingSettings {
        #[default]
        NoProject,
        Project {
            selected_local_folder: String,
            initialize_projects_automatically: bool,
        },
    }

    impl ProjectOnboardingSettings {
        pub fn from_path(path: Option<String>) -> Self {
            match path {
                Some(path) => Self::Project {
                    selected_local_folder: path,
                    initialize_projects_automatically: true,
                },
                None => Self::NoProject,
            }
        }
    }
}

pub use slides::ProjectOnboardingSettings;

#[derive(Clone, Debug)]
pub struct UICustomizationSettings {
    pub use_vertical_tabs: bool,
    pub show_conversation_history: bool,
    pub show_project_explorer: bool,
    pub show_global_search: bool,
    pub show_warp_drive: bool,
    pub show_code_review_button: bool,
}

impl UICustomizationSettings {
    pub fn agent_defaults() -> Self {
        Self {
            use_vertical_tabs: true,
            show_conversation_history: true,
            show_project_explorer: true,
            show_global_search: true,
            show_warp_drive: true,
            show_code_review_button: true,
        }
    }

    pub fn terminal_defaults() -> Self {
        Self {
            use_vertical_tabs: false,
            show_conversation_history: false,
            show_project_explorer: false,
            show_global_search: false,
            show_warp_drive: false,
            show_code_review_button: false,
        }
    }

    pub fn tools_panel_enabled(&self, intention: &OnboardingIntention) -> bool {
        let conversation_visible = matches!(intention, OnboardingIntention::AgentDrivenDevelopment);
        (conversation_visible && self.show_conversation_history)
            || self.show_project_explorer
            || self.show_global_search
            || self.show_warp_drive
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OnboardingAuthState {
    LoggedOut,
    FreeUser,
    PayingUser,
}

#[derive(Clone, Debug)]
pub enum SelectedSettings {
    Terminal {
        ui_customization: Option<UICustomizationSettings>,
        cli_agent_toolbar_enabled: bool,
        show_agent_notifications: bool,
    },
    AgentDrivenDevelopment {
        agent_settings: slides::AgentDevelopmentSettings,
        project_settings: slides::ProjectOnboardingSettings,
        ui_customization: Option<UICustomizationSettings>,
    },
}

impl SelectedSettings {
    pub fn is_ai_enabled(&self) -> bool {
        match self {
            Self::AgentDrivenDevelopment { agent_settings, .. } => !agent_settings.disable_oz,
            Self::Terminal { .. } => false,
        }
    }

    pub fn is_warp_drive_enabled(&self) -> bool {
        match self {
            Self::AgentDrivenDevelopment {
                ui_customization, ..
            } => ui_customization
                .as_ref()
                .map(|ui| ui.show_warp_drive)
                .unwrap_or(true),
            Self::Terminal {
                ui_customization, ..
            } => ui_customization
                .as_ref()
                .map(|ui| ui.show_warp_drive)
                .unwrap_or(false),
        }
    }
}

#[derive(Clone, Debug)]
pub enum OnboardingEvent {
    OnboardingStarted,
}

#[derive(Clone, Debug)]
pub enum AgentOnboardingEvent {
    ThemeSelected { theme_name: String },
    SyncWithOsToggled { enabled: bool },
    OnboardingCompleted(SelectedSettings),
    OnboardingSkipped,
    LoginFromWelcomeRequested,
    PrivacySettingsFromTerminalThemeSlideRequested,
    UpgradeRequested,
    UpgradeCopyUrlRequested,
    UpgradePasteTokenFromClipboardRequested,
    AppBecameActive,
}

pub struct AgentOnboardingView {
    use_vertical_tabs: bool,
    free_user_no_ai_experiment: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum AgentOnboardingAction {
    UpKey,
    DownKey,
    LeftKey,
    RightKey,
    TabKey,
    EnterKey,
    CmdOrCtrlEnterKey,
    Escape,
}

impl AgentOnboardingView {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        _theme_picker_themes: [WarpTheme; 4],
        _skippable: bool,
        _models: Vec<slides::OnboardingModelInfo>,
        _default_model_id: LLMId,
        _workspace_enforces_autonomy: bool,
        _agent_modality_enabled: bool,
        free_user_no_ai_experiment: bool,
        _agent_price_cents: Option<i32>,
        _auth_state: OnboardingAuthState,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            use_vertical_tabs: false,
            free_user_no_ai_experiment,
        }
    }

    pub fn set_onboarding_models(
        &mut self,
        _models: Vec<slides::OnboardingModelInfo>,
        _default_model_id: LLMId,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn set_workspace_enforces_autonomy(&mut self, _value: bool, _ctx: &mut ViewContext<Self>) {}

    pub fn set_auth_state(
        &mut self,
        _auth_state: OnboardingAuthState,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn free_user_no_ai_experiment(&self, _ctx: &AppContext) -> bool {
        self.free_user_no_ai_experiment
    }

    pub fn use_vertical_tabs(&self, _ctx: &AppContext) -> bool {
        self.use_vertical_tabs
    }

    pub fn set_agent_price_cents(&mut self, _cents: Option<i32>, _ctx: &mut ViewContext<Self>) {}

    pub fn set_free_user_no_ai_experiment(&mut self, value: bool, _ctx: &mut ViewContext<Self>) {
        self.free_user_no_ai_experiment = value;
    }

    pub fn advance_to_agent_step(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn start_onboarding(&self, ctx: &mut ViewContext<Self>) {
        ctx.emit(AgentOnboardingEvent::OnboardingSkipped);
    }
}

impl Entity for AgentOnboardingView {
    type Event = AgentOnboardingEvent;
}

impl View for AgentOnboardingView {
    fn ui_name() -> &'static str {
        "AgentOnboardingView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for AgentOnboardingView {
    type Action = AgentOnboardingAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl SingletonEntity for AgentOnboardingView {}

pub mod callout {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct OnboardingKeybindings {
        pub toggle_input_mode: String,
        pub submit_to_local_agent: String,
        pub submit_to_cloud_agent: String,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FinalState {
        Submit,
        Skip,
        Finish,
        Initialize,
        BackToTerminal,
    }

    impl std::fmt::Display for FinalState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Submit => write!(f, "submitted"),
                Self::Skip => write!(f, "skipped"),
                Self::Finish => write!(f, "finished"),
                Self::Initialize => write!(f, "initialize"),
                Self::BackToTerminal => write!(f, "back_to_terminal"),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum OnboardingQuery {
        TerminalCommand(String),
        AgentPrompt(String),
        None,
    }

    #[derive(Clone, Debug)]
    pub enum OnboardingCalloutViewEvent {
        StateUpdated,
        Completed { final_state: FinalState },
        EnterAgentModality,
        NaturalLanguageDetectionToggled(bool),
    }

    #[derive(Clone, Debug)]
    pub enum OnboardingCalloutViewAction {
        NextClicked,
        SkipClicked,
        BackToTerminalClicked,
        ToggleCheckbox,
    }

    pub struct OnboardingCalloutView {
        has_project: bool,
    }

    impl OnboardingCalloutView {
        pub fn new_universal_input(
            has_project: bool,
            _initial_natural_language_detection_enabled: bool,
            _keybindings: OnboardingKeybindings,
            _ctx: &mut ViewContext<Self>,
        ) -> Self {
            Self { has_project }
        }

        pub fn new_agent_modality(
            has_project: bool,
            _intention: OnboardingIntention,
            _initial_natural_language_detection_enabled: bool,
            _keybindings: OnboardingKeybindings,
            _ctx: &mut ViewContext<Self>,
        ) -> Self {
            Self { has_project }
        }

        pub fn has_project(&self, _app: &AppContext) -> bool {
            self.has_project
        }

        pub fn start_onboarding(&mut self, ctx: &mut ViewContext<Self>) {
            ctx.emit(OnboardingCalloutViewEvent::Completed {
                final_state: FinalState::Finish,
            });
        }

        pub fn is_onboarding_active(&self, _app: &AppContext) -> bool {
            false
        }

        pub fn prompt_string(&self, _app: &AppContext) -> String {
            String::new()
        }

        pub fn prompt(&self, _app: &AppContext) -> OnboardingQuery {
            OnboardingQuery::None
        }

        pub fn should_position_above_zero_state(&self, _app: &AppContext) -> bool {
            false
        }
    }

    impl Entity for OnboardingCalloutView {
        type Event = OnboardingCalloutViewEvent;
    }

    impl View for OnboardingCalloutView {
        fn ui_name() -> &'static str {
            "OnboardingCalloutView"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }

    impl TypedActionView for OnboardingCalloutView {
        type Action = OnboardingCalloutViewAction;

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }

    impl SingletonEntity for OnboardingCalloutView {}
}

pub use callout::{OnboardingCalloutView, OnboardingKeybindings};

pub fn init(_app: &mut AppContext) {}
