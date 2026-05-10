use std::{collections::HashMap, path::PathBuf};

use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use settings::{
    define_settings_group, RespectUserSyncSetting, Setting, SupportedPlatforms, SyncToCloud,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use warpui::{
    platform::{keyboard::KeyCode, OperatingSystem},
    AppContext, Entity, ModelContext, SingletonEntity,
};

use crate::{ai::request_usage_model::RequestLimitInfo, terminal::CLIAgent};

pub enum FocusedTerminalInfoEvent {
    TerminalInfoUpdated,
}

#[derive(Default, Clone, Debug)]
pub struct FocusedTerminalInfo;

impl FocusedTerminalInfo {
    pub fn new(_: &mut ModelContext<Self>) -> Self {
        Self
    }

    pub fn contains_any_remote_blocks(&self) -> bool {
        false
    }

    pub fn contains_any_restored_remote_blocks(&self) -> bool {
        false
    }

    pub fn update(
        &mut self,
        _contains_any_remote_blocks: bool,
        _contains_any_restored_remote_blocks: bool,
        _ctx: &mut ModelContext<Self>,
    ) -> bool {
        false
    }
}

impl Entity for FocusedTerminalInfo {
    type Event = FocusedTerminalInfoEvent;
}

impl SingletonEntity for FocusedTerminalInfo {}

#[derive(
    Default,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Copy,
    Clone,
    EnumIter,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[schemars(rename_all = "snake_case")]
pub enum VoiceInputToggleKey {
    #[default]
    None,
    Fn,
    AltLeft,
    AltRight,
    ControlLeft,
    ControlRight,
    SuperLeft,
    SuperRight,
    ShiftLeft,
    ShiftRight,
}

settings::macros::implement_setting_for_enum!(
    VoiceInputToggleKey,
    AISettings,
    SupportedPlatforms::DESKTOP,
    SyncToCloud::Never,
    private: true,
);

impl VoiceInputToggleKey {
    pub fn all_possible_values() -> Vec<VoiceInputToggleKey> {
        let all_keys = VoiceInputToggleKey::iter().collect();
        match OperatingSystem::get() {
            OperatingSystem::Mac => all_keys,
            OperatingSystem::Windows | OperatingSystem::Linux | OperatingSystem::Other(_) => {
                all_keys
                    .into_iter()
                    .filter(|key| *key != VoiceInputToggleKey::Fn)
                    .collect()
            }
        }
    }

    pub fn display_name(&self) -> &'static str {
        let (super_key_name, alt_key_name): (&'static str, &'static str) =
            match OperatingSystem::get() {
                OperatingSystem::Mac => ("Command", "Option"),
                OperatingSystem::Windows => ("Windows", "Alt"),
                OperatingSystem::Linux | OperatingSystem::Other(_) => ("Super", "Alt"),
            };

        match self {
            VoiceInputToggleKey::None => "None",
            VoiceInputToggleKey::Fn => "Fn",
            VoiceInputToggleKey::AltLeft => {
                Box::leak(format!("{alt_key_name} (Left)").into_boxed_str())
            }
            VoiceInputToggleKey::AltRight => {
                Box::leak(format!("{alt_key_name} (Right)").into_boxed_str())
            }
            VoiceInputToggleKey::ControlLeft => "Control (Left)",
            VoiceInputToggleKey::ControlRight => "Control (Right)",
            VoiceInputToggleKey::SuperLeft => {
                Box::leak(format!("{super_key_name} (Left)").into_boxed_str())
            }
            VoiceInputToggleKey::SuperRight => {
                Box::leak(format!("{super_key_name} (Right)").into_boxed_str())
            }
            VoiceInputToggleKey::ShiftLeft => "Shift (Left)",
            VoiceInputToggleKey::ShiftRight => "Shift (Right)",
        }
    }

    pub fn to_key_code(&self) -> Option<KeyCode> {
        match self {
            VoiceInputToggleKey::None => None,
            VoiceInputToggleKey::Fn => Some(KeyCode::Fn),
            VoiceInputToggleKey::AltLeft => Some(KeyCode::AltLeft),
            VoiceInputToggleKey::AltRight => Some(KeyCode::AltRight),
            VoiceInputToggleKey::ControlLeft => Some(KeyCode::ControlLeft),
            VoiceInputToggleKey::ControlRight => Some(KeyCode::ControlRight),
            VoiceInputToggleKey::SuperLeft => Some(KeyCode::SuperLeft),
            VoiceInputToggleKey::SuperRight => Some(KeyCode::SuperRight),
            VoiceInputToggleKey::ShiftLeft => Some(KeyCode::ShiftLeft),
            VoiceInputToggleKey::ShiftRight => Some(KeyCode::ShiftRight),
        }
    }

    pub fn keystroke(&self) -> Option<warpui::keymap::Keystroke> {
        use warpui::keymap::Keystroke;

        let keystroke = match self {
            VoiceInputToggleKey::None => return None,
            VoiceInputToggleKey::Fn => Keystroke {
                key: "fn".to_string(),
                ..Default::default()
            },
            VoiceInputToggleKey::AltLeft | VoiceInputToggleKey::AltRight => Keystroke {
                alt: true,
                ..Default::default()
            },
            VoiceInputToggleKey::ControlLeft | VoiceInputToggleKey::ControlRight => Keystroke {
                ctrl: true,
                ..Default::default()
            },
            VoiceInputToggleKey::SuperLeft | VoiceInputToggleKey::SuperRight => Keystroke {
                cmd: true,
                ..Default::default()
            },
            VoiceInputToggleKey::ShiftLeft | VoiceInputToggleKey::ShiftRight => Keystroke {
                shift: true,
                ..Default::default()
            },
        };
        Some(keystroke)
    }

    pub fn tooltip_message(&self) -> String {
        match self.keystroke() {
            Some(keystroke) => {
                let symbol = keystroke.displayed();
                let side = match self {
                    VoiceInputToggleKey::AltLeft
                    | VoiceInputToggleKey::ControlLeft
                    | VoiceInputToggleKey::SuperLeft
                    | VoiceInputToggleKey::ShiftLeft => Some("Left"),
                    VoiceInputToggleKey::AltRight
                    | VoiceInputToggleKey::ControlRight
                    | VoiceInputToggleKey::SuperRight
                    | VoiceInputToggleKey::ShiftRight => Some("Right"),
                    VoiceInputToggleKey::None | VoiceInputToggleKey::Fn => None,
                };
                let key_name = match side {
                    Some(side) => format!("{side} {symbol}"),
                    None => symbol,
                };
                format!("Voice input (hold {key_name} key)")
            }
            None => "Voice input".to_string(),
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, VoiceInputToggleKey::None)
    }
}

#[derive(
    Default,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Copy,
    Clone,
    EnumIter,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[schemars(rename_all = "snake_case")]
pub enum DefaultSessionMode {
    #[default]
    Terminal,
    Agent,
    CloudAgent,
    TabConfig,
    DockerSandbox,
}

settings::macros::implement_setting_for_enum!(
    DefaultSessionMode,
    AISettings,
    SupportedPlatforms::ALL,
    SyncToCloud::Globally(RespectUserSyncSetting::Yes),
    private: false,
    toml_path: "general.default_session_mode",
);

impl DefaultSessionMode {
    pub fn display_name(&self) -> &'static str {
        match self {
            DefaultSessionMode::Terminal => "Terminal",
            DefaultSessionMode::Agent => "Agent",
            DefaultSessionMode::CloudAgent => "Cloud Oz",
            DefaultSessionMode::TabConfig => "Tab Config",
            DefaultSessionMode::DockerSandbox => "Local Docker Sandbox",
        }
    }
}

#[derive(
    Default,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Copy,
    Clone,
    EnumIter,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[schemars(rename_all = "snake_case")]
pub enum ThinkingDisplayMode {
    #[default]
    ShowAndCollapse,
    AlwaysShow,
    NeverShow,
}

settings::macros::implement_setting_for_enum!(
    ThinkingDisplayMode,
    AISettings,
    SupportedPlatforms::ALL,
    SyncToCloud::Globally(RespectUserSyncSetting::Yes),
    private: true,
);

impl ThinkingDisplayMode {
    pub fn display_name(&self) -> &'static str {
        match self {
            ThinkingDisplayMode::ShowAndCollapse => "Show & collapse",
            ThinkingDisplayMode::AlwaysShow => "Always show",
            ThinkingDisplayMode::NeverShow => "Never show",
        }
    }

    pub fn command_palette_description(&self) -> &'static str {
        match self {
            ThinkingDisplayMode::ShowAndCollapse => "Set agent thinking display: show & collapse",
            ThinkingDisplayMode::AlwaysShow => "Set agent thinking display: always show",
            ThinkingDisplayMode::NeverShow => "Set agent thinking display: never show",
        }
    }

    pub fn should_render(&self) -> bool {
        !matches!(self, ThinkingDisplayMode::NeverShow)
    }

    pub fn should_keep_expanded(&self) -> bool {
        matches!(self, ThinkingDisplayMode::AlwaysShow)
    }
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Default,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
pub struct BannerState {
    #[serde(default)]
    pub dismissed: bool,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
pub struct CycleInfo {
    pub end_date: DateTime<Utc>,
    pub was_quota_exceeded: bool,
    pub banner_state: BannerState,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Default,
    PartialEq,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
pub struct AIRequestQuotaInfo {
    pub cycle_history: Vec<CycleInfo>,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Default,
    PartialEq,
    EnumIter,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[schemars(rename_all = "snake_case")]
pub enum AgentModeCodingPermissionsType {
    #[default]
    AlwaysAskBeforeReading,
    AlwaysAllowReading,
    AllowReadingSpecificFiles,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum AgentModeCommandExecutionPredicateType {
    #[serde(with = "serde_regex")]
    AnchoredRegex(Regex),
}

impl AgentModeCommandExecutionPredicateType {
    fn new_regex(regex: &str) -> Result<Self, regex::Error> {
        Ok(Self::AnchoredRegex(Regex::new(&format!("^{regex}$"))?))
    }

    fn matches(&self, cmd: &str) -> bool {
        match self {
            Self::AnchoredRegex(regex) => regex.is_match(cmd),
        }
    }
}

impl PartialEq for AgentModeCommandExecutionPredicateType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::AnchoredRegex(a), Self::AnchoredRegex(b)) => {
                let a = &a.as_str()[1..a.as_str().len() - 1];
                let b = &b.as_str()[1..b.as_str().len() - 1];
                a == b
            }
        }
    }
}

impl std::fmt::Display for AgentModeCommandExecutionPredicateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnchoredRegex(regex) => {
                write!(f, "{}", &regex.as_str()[1..regex.as_str().len() - 1])
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct AgentModeCommandExecutionPredicate(AgentModeCommandExecutionPredicateType);

impl schemars::JsonSchema for AgentModeCommandExecutionPredicate {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("AgentModeCommandExecutionPredicate")
    }

    fn json_schema(gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        gen.subschema_for::<String>()
    }
}

impl AgentModeCommandExecutionPredicate {
    pub fn new_regex(regex: &str) -> Result<Self, regex::Error> {
        Ok(Self(AgentModeCommandExecutionPredicateType::new_regex(
            regex,
        )?))
    }

    pub fn matches(&self, cmd: &str) -> bool {
        self.0.matches(cmd)
    }
}

impl std::fmt::Display for AgentModeCommandExecutionPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl settings_value::SettingsValue for AgentModeCommandExecutionPredicate {
    fn to_file_value(&self) -> serde_json::Value {
        serde_json::Value::String(self.to_string())
    }

    fn from_file_value(value: &serde_json::Value) -> Option<Self> {
        value.as_str().and_then(|s| Self::new_regex(s).ok())
    }
}

lazy_static! {
    pub static ref DEFAULT_COMMAND_EXECUTION_ALLOWLIST: Vec<AgentModeCommandExecutionPredicate> =
        vec![];
    pub static ref DEFAULT_COMMAND_EXECUTION_DENYLIST: Vec<AgentModeCommandExecutionPredicate> =
        vec![];
}

define_settings_group!(AISettings, settings: [
    is_any_ai_enabled: IsAnyAIEnabled {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    is_active_ai_enabled_internal: IsActiveAIEnabled {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    ai_autodetection_enabled_internal: AIAutoDetectionEnabled {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    nld_in_terminal_enabled_internal: NLDInTerminalEnabled {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    autodetection_command_denylist: AICommandDenylist {
        type: String,
        default: String::new(),
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    prompt_suggestions_enabled_internal: AgentModeQuerySuggestionsEnabled {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    rule_suggestions_enabled_internal: RuleSuggestionsEnabled {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    entered_agent_mode_num_times: EnteredAgentModeNumTimes {
        type: usize,
        default: 0,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    dismissed_voice_input_new_feature_popup: DismissedVoiceInputNewFeaturePopup {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::DESKTOP,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    voice_input_toggle_key: VoiceInputToggleKey,
    explicitly_interacted_with_voice: ExplicitlyInteractedWithVoice {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::DESKTOP,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    agent_mode_command_execution_allowlist: AgentModeCommandExecutionAllowlist {
        type: Vec<AgentModeCommandExecutionPredicate>,
        default: vec![],
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    agent_mode_command_execution_denylist: AgentModeCommandExecutionDenylist {
        type: Vec<AgentModeCommandExecutionPredicate>,
        default: vec![],
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    agent_mode_execute_read_only_commands: AgentModeExecuteReadonlyCommands {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    agent_mode_coding_permissions: AgentModeCodingPermissions {
        type: AgentModeCodingPermissionsType,
        default: AgentModeCodingPermissionsType::default(),
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    agent_mode_coding_file_read_allowlist: AgentModeCodingFileReadAllowlist {
        type: Vec<PathBuf>,
        default: vec![],
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    has_shown_agent_mode_profile_command_autoexecution_speedbump: HasShownAgentModeProfileCommandAutoexecutionSpeedbump {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    should_show_agent_mode_autoexecute_readonly_commands_speedbump: ShouldShowAgentModeModelExecuteReadonlyCommandsSpeedbump {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    should_show_agent_mode_write_to_pty_speedbump: ShouldShowAgentModeWriteToPtySpeedbump {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    should_show_agent_mode_autoread_files_speedbump: ShouldShowAgentModeCodingReadPermissionsNudge {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    aws_bedrock_credentials_enabled: AwsBedrockCredentialsEnabled {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::DESKTOP,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    aws_bedrock_auto_login: AwsBedrockAutoLogin {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::DESKTOP,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    aws_bedrock_auth_refresh_command: AwsBedrockAuthRefreshCommand {
        type: String,
        default: String::new(),
        supported_platforms: SupportedPlatforms::DESKTOP,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    aws_bedrock_profile: AwsBedrockProfile {
        type: String,
        default: String::new(),
        supported_platforms: SupportedPlatforms::DESKTOP,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    aws_bedrock_login_banner_dismissed: AwsBedrockLoginBannerDismissed {
        type: bool,
        default: true,
        supported_platforms: SupportedPlatforms::DESKTOP,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    memory_enabled: MemoryEnabled {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    warp_drive_context_enabled: WarpDriveContextEnabled {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    codebase_index_speedbump_banner_dismissed_for_repo_paths: CodebaseIndexSpeedbumpBannerDismissedForRepoPaths {
        type: Vec<PathBuf>,
        default: vec![],
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    agent_mode_setup_banner_shown_for_repo_paths: AgentModeSetupBannerShownForRepoPaths {
        type: Vec<PathBuf>,
        default: vec![],
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    codebase_index_speedbump_banner_globally_dismissed: CodebaseIndexSpeedbumpBannerGloballyDismissed {
        type: bool,
        default: true,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    ai_request_quota_info: AIRequestQuotaInfoSetting {
        type: AIRequestQuotaInfo,
        default: AIRequestQuotaInfo::default(),
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    show_code_suggestion_speedbump: ShouldShowCodeSuggestionSpeedbump {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    mcp_execution_path: MCPExecutionPath {
        type: Option<String>,
        default: None,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    did_check_to_trigger_agents_3_launch_modal: DidShowAgents3LaunchModal {
        type: bool,
        default: true,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    did_check_to_trigger_oz_launch_modal: DidCheckToTriggerLaunchModal {
        type: bool,
        default: true,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    should_expand_oz_updates: ShouldExpandOzUpdates {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    should_show_oz_updates_in_zero_state: ShouldShowOzUpdatesInZeroState {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    can_use_warp_credits_with_byok: CanUseWarpCreditsWithByok {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    has_auto_opened_conversation_list: HasAutoOpenedConversationList {
        type: bool,
        default: true,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    default_session_mode_internal: DefaultSessionMode,
    default_tab_config_path: DefaultTabConfigPath {
        type: String,
        default: String::new(),
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: false,
        toml_path: "general.default_tab_config_path",
    },
    cloud_agent_computer_use_enabled: CloudAgentComputerUseEnabled {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::DESKTOP,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    orchestration_enabled: OrchestrationEnabled {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::DESKTOP,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    file_based_mcp_enabled: FileBasedMcpEnabled {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::DESKTOP,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    thinking_display_mode: ThinkingDisplayMode,
    include_agent_commands_in_history: IncludeAgentCommandsInHistory {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    show_conversation_history: ShowConversationHistory {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    show_agent_notifications: ShowAgentNotifications {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
]);

impl AISettings {
    pub fn register_and_subscribe_to_events(app: &mut AppContext) {
        Self::register(app);
        app.add_singleton_model(FocusedTerminalInfo::new);
    }

    pub fn is_ai_disabled_due_to_remote_session_org_policy(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_any_ai_enabled(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn default_session_mode(&self, _app: &AppContext) -> DefaultSessionMode {
        match *self.default_session_mode_internal.value() {
            DefaultSessionMode::TabConfig => DefaultSessionMode::TabConfig,
            DefaultSessionMode::DockerSandbox => DefaultSessionMode::DockerSandbox,
            _ => DefaultSessionMode::Terminal,
        }
    }

    pub fn default_tab_config_path(&self) -> &str {
        &self.default_tab_config_path
    }

    pub fn resolved_default_tab_config(
        &self,
        app: &AppContext,
    ) -> Option<crate::tab_configs::TabConfig> {
        let path_str = self.default_tab_config_path.as_str();
        if path_str.is_empty() {
            return None;
        }
        let path = std::path::Path::new(path_str);
        crate::user_config::WarpConfig::as_ref(app)
            .tab_configs()
            .iter()
            .find(|config| config.source_path.as_deref().is_some_and(|p| p == path))
            .cloned()
    }

    pub fn is_active_ai_enabled(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_prompt_suggestions_enabled(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_rule_suggestions_enabled(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_code_suggestions_enabled(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_natural_language_autosuggestions_enabled(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_shared_block_title_generation_enabled(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_git_operations_autogen_enabled(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_intelligent_autosuggestions_enabled(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_voice_input_enabled(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_ai_autodetection_enabled(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_nld_in_terminal_enabled(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_memory_enabled(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_warp_drive_context_enabled(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_file_based_mcp_enabled(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_orchestration_enabled(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn should_display_quota_reset_banner(&self) -> bool {
        false
    }

    pub fn mark_quota_banner_as_dismissed(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn update_quota_info(
        &mut self,
        _request_limit_info: &RequestLimitInfo,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn is_command_denylist_editable(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_command_allowlist_editable(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_directory_allowlist_editable(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_execute_commands_permissions_editable(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_write_to_pty_permissions_editable(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_computer_use_permissions_editable(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_read_files_permissions_editable(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_code_diffs_permissions_editable(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_ask_user_question_permissions_editable(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_mcp_permission_editable(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn show_code_suggestion_speedbump(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn maybe_setup_first_time_voice(
        &mut self,
        _ctx: &mut ModelContext<Self>,
    ) -> Option<VoiceInputToggleKey> {
        None
    }

    pub fn add_cli_agent_footer_enabled_command(
        &mut self,
        _command: &str,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn remove_cli_agent_footer_enabled_command(
        &mut self,
        _command: &str,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn set_cli_agent_for_command(
        &mut self,
        _pattern: &str,
        _agent: Option<CLIAgent>,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn is_plugin_install_chip_dismissed(&self, _key: &str) -> bool {
        true
    }

    pub fn dismiss_plugin_install_chip(&mut self, _key: &str, _ctx: &mut ModelContext<Self>) {}

    pub fn plugin_update_chip_dismissed_version(&self, _key: &str) -> &str {
        ""
    }

    pub fn dismiss_plugin_update_chip(
        &mut self,
        _key: &str,
        _version: String,
        _ctx: &mut ModelContext<Self>,
    ) {
    }
}

pub struct CompiledCommandsForCodingAgentToolbar;

impl CompiledCommandsForCodingAgentToolbar {
    pub fn matched_agent(_app: &AppContext, _command: &str) -> Option<CLIAgent> {
        None
    }
}

impl Entity for CompiledCommandsForCodingAgentToolbar {
    type Event = ();
}

impl SingletonEntity for CompiledCommandsForCodingAgentToolbar {}
