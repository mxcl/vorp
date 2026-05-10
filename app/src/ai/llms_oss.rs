use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use warp_core::ui::icons::Icon;
use warpui::{AppContext, Entity, EntityId, ModelContext, SingletonEntity};

pub use ai::LLMId;

pub fn is_using_api_key_for_provider(_provider: &LLMProvider, _app: &AppContext) -> bool {
    false
}

pub const MODELS_BY_FEATURE_CACHE_KEY: &str = "AvailableLLMs";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LLMUsageMetadata {
    pub request_multiplier: usize,
    pub credit_multiplier: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisableReason {
    AdminDisabled,
    OutOfRequests,
    ProviderOutage,
    RequiresUpgrade,
    Unavailable,
}

impl DisableReason {
    pub fn tooltip_text(&self) -> &'static str {
        "This model is unavailable."
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LLMSpec {
    pub cost: f32,
    pub quality: f32,
    pub speed: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LLMProvider {
    OpenAI,
    Anthropic,
    Google,
    Xai,
    Unknown,
}

impl LLMProvider {
    pub fn icon(&self) -> Option<Icon> {
        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LLMModelHost {
    DirectApi,
    AwsBedrock,
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingHostConfig {
    pub enabled: bool,
    pub model_routing_host: LLMModelHost,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LLMContextWindow {
    #[serde(default)]
    pub is_configurable: bool,
    #[serde(default)]
    pub min: u32,
    #[serde(default)]
    pub max: u32,
    #[serde(default)]
    pub default_max: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LLMInfo {
    pub display_name: String,
    pub base_model_name: String,
    pub id: LLMId,
    pub reasoning_level: Option<String>,
    pub usage_metadata: LLMUsageMetadata,
    pub description: Option<String>,
    pub disable_reason: Option<DisableReason>,
    pub vision_supported: bool,
    pub spec: Option<LLMSpec>,
    pub provider: LLMProvider,
    pub host_configs: HashMap<LLMModelHost, RoutingHostConfig>,
    pub discount_percentage: Option<f32>,
    pub context_window: LLMContextWindow,
}

pub fn dedupe_model_display_names<'a>(
    choices: impl IntoIterator<Item = &'a LLMInfo>,
) -> Vec<String> {
    let names: HashSet<String> = choices
        .into_iter()
        .map(|choice| choice.base_model_name.clone())
        .collect();
    let mut sorted: Vec<String> = names.into_iter().collect();
    sorted.sort();
    sorted
}

impl LLMInfo {
    pub fn menu_display_name(&self) -> String {
        self.display_name.clone()
    }

    pub fn base_model_name(&self) -> &str {
        &self.base_model_name
    }

    pub fn has_reasoning_level(&self) -> bool {
        self.reasoning_level.is_some()
    }

    pub fn reasoning_level(&self) -> Option<String> {
        self.reasoning_level.clone()
    }

    #[cfg(feature = "integration_tests")]
    fn new_for_test(llm_name: &str) -> Self {
        model_info(llm_name, llm_name, false)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableLLMs {
    default_id: LLMId,
    choices: Vec<LLMInfo>,
    #[serde(default)]
    preferred_codex_model_id: Option<LLMId>,
}

impl AvailableLLMs {
    pub fn new<T: Into<LLMInfo>>(
        mut default_id: LLMId,
        choices: impl IntoIterator<Item = T>,
        preferred_codex_model_id: Option<LLMId>,
    ) -> Result<Self, anyhow::Error> {
        let choices: Vec<LLMInfo> = choices.into_iter().map(Into::into).collect();
        if choices.is_empty() {
            return Err(anyhow::anyhow!("No models are available."));
        }

        if !choices.iter().any(|info| info.id == default_id) {
            default_id = choices[0].id.clone();
        }

        Ok(Self {
            default_id,
            choices,
            preferred_codex_model_id,
        })
    }

    fn info_for_id(&self, id: &LLMId) -> Option<&LLMInfo> {
        self.choices.iter().find(|info| info.id == *id)
    }

    fn default_llm_info(&self) -> &LLMInfo {
        self.info_for_id(&self.default_id)
            .unwrap_or_else(|| &self.choices[0])
    }

    #[cfg(feature = "integration_tests")]
    pub fn new_for_test(llm_name: &str) -> Self {
        Self {
            default_id: llm_name.into(),
            choices: vec![LLMInfo::new_for_test(llm_name)],
            preferred_codex_model_id: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelsByFeature {
    pub agent_mode: AvailableLLMs,
    pub coding: AvailableLLMs,
    #[serde(default)]
    pub cli_agent: Option<AvailableLLMs>,
    #[serde(default)]
    pub computer_use: Option<AvailableLLMs>,
}

impl ModelsByFeature {
    fn info_for_id(&self, id: &LLMId) -> Option<&LLMInfo> {
        self.agent_mode
            .info_for_id(id)
            .or_else(|| self.coding.info_for_id(id))
            .or_else(|| {
                self.cli_agent
                    .as_ref()
                    .and_then(|llms| llms.info_for_id(id))
            })
            .or_else(|| {
                self.computer_use
                    .as_ref()
                    .and_then(|llms| llms.info_for_id(id))
            })
    }
}

impl Default for ModelsByFeature {
    fn default() -> Self {
        Self {
            agent_mode: available_llms("oss-disabled", "Unavailable", false),
            coding: available_llms("oss-disabled", "Unavailable", false),
            cli_agent: Some(available_llms("oss-disabled", "Unavailable", false)),
            computer_use: Some(available_llms("oss-disabled", "Unavailable", false)),
        }
    }
}

pub struct LLMPreferences {
    models_by_feature: ModelsByFeature,
    base_llm_for_terminal_view: HashMap<EntityId, LLMId>,
}

impl LLMPreferences {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self {
            models_by_feature: ModelsByFeature::default(),
            base_llm_for_terminal_view: HashMap::new(),
        }
    }

    pub fn get_active_base_model<'a>(
        &'a self,
        _app: &'a AppContext,
        terminal_view_id: Option<EntityId>,
    ) -> &'a LLMInfo {
        terminal_view_id
            .and_then(|id| self.base_llm_for_terminal_view.get(&id))
            .and_then(|id| self.models_by_feature.agent_mode.info_for_id(id))
            .unwrap_or_else(|| self.models_by_feature.agent_mode.default_llm_info())
    }

    pub fn get_active_coding_model<'a>(
        &'a self,
        _app: &'a AppContext,
        _terminal_view_id: Option<EntityId>,
    ) -> &'a LLMInfo {
        self.models_by_feature.coding.default_llm_info()
    }

    pub fn get_base_llm_choices_for_agent_mode(&self) -> impl Iterator<Item = &LLMInfo> {
        self.models_by_feature.agent_mode.choices.iter()
    }

    pub fn get_coding_llm_choices(&self) -> impl Iterator<Item = &LLMInfo> {
        self.models_by_feature.coding.choices.iter()
    }

    pub fn get_cli_agent_llm_choices(&self) -> impl Iterator<Item = &LLMInfo> {
        self.models_by_feature
            .cli_agent
            .as_ref()
            .into_iter()
            .flat_map(|llms| llms.choices.iter())
    }

    pub fn get_active_cli_agent_model<'a>(
        &'a self,
        _app: &'a AppContext,
        _terminal_view_id: Option<EntityId>,
    ) -> &'a LLMInfo {
        self.get_default_cli_agent_model()
    }

    pub fn get_default_cli_agent_model(&self) -> &LLMInfo {
        self.models_by_feature
            .cli_agent
            .as_ref()
            .unwrap_or(&self.models_by_feature.agent_mode)
            .default_llm_info()
    }

    pub fn get_computer_use_llm_choices(&self) -> impl Iterator<Item = &LLMInfo> {
        self.models_by_feature
            .computer_use
            .as_ref()
            .into_iter()
            .flat_map(|llms| llms.choices.iter())
    }

    pub fn get_active_computer_use_model<'a>(
        &'a self,
        _app: &'a AppContext,
        _terminal_view_id: Option<EntityId>,
    ) -> &'a LLMInfo {
        self.get_default_computer_use_model()
    }

    pub fn get_default_computer_use_model(&self) -> &LLMInfo {
        self.models_by_feature
            .computer_use
            .as_ref()
            .unwrap_or(&self.models_by_feature.agent_mode)
            .default_llm_info()
    }

    pub fn get_llm_info(&self, id: &LLMId) -> Option<&LLMInfo> {
        self.models_by_feature.info_for_id(id)
    }

    pub fn get_default_base_model(&self) -> &LLMInfo {
        self.models_by_feature.agent_mode.default_llm_info()
    }

    pub fn get_default_coding_model(&self) -> &LLMInfo {
        self.models_by_feature.coding.default_llm_info()
    }

    pub fn get_preferred_codex_model(&self) -> Option<&LLMInfo> {
        None
    }

    #[cfg(feature = "integration_tests")]
    pub fn is_available_agent_mode_llm(&self, id: &LLMId) -> bool {
        self.models_by_feature.agent_mode.info_for_id(id).is_some()
    }

    pub fn update_preferred_agent_mode_llm(
        &mut self,
        preferred_llm_id: &LLMId,
        terminal_view_id: EntityId,
        ctx: &mut ModelContext<Self>,
    ) {
        self.base_llm_for_terminal_view
            .insert(terminal_view_id, preferred_llm_id.clone());
        ctx.emit(LLMPreferencesEvent::UpdatedActiveAgentModeLLM);
    }

    pub fn update_preferred_coding_llm(
        &self,
        _preferred_llm_id: &LLMId,
        _terminal_view_id: Option<EntityId>,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn new_choices_since_last_update(&self) -> Option<Vec<LLMInfo>> {
        None
    }

    pub fn should_show_new_choices_popup(&self, _view_id: EntityId) -> bool {
        false
    }

    pub fn mark_new_choices_popup_as_shown(&self, _view_id: EntityId) {}

    pub fn hide_llm_popup(&self, _view_id: EntityId) {}

    pub fn refresh_authed_models(&self, _ctx: &mut ModelContext<Self>) {}

    pub fn refresh_available_models(&self, _ctx: &mut ModelContext<Self>) {}

    pub fn update_feature_model_choices(
        &mut self,
        _choices_result: Result<ModelsByFeature, anyhow::Error>,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn vision_supported(&self, _app: &AppContext, _terminal_view_id: Option<EntityId>) -> bool {
        false
    }

    pub fn get_base_llm_override(&self, terminal_view_id: EntityId) -> Option<String> {
        self.base_llm_for_terminal_view
            .get(&terminal_view_id)
            .and_then(|llm_id| serde_json::to_string(llm_id).ok())
    }

    pub fn remove_llm_override(
        &mut self,
        terminal_view_id: EntityId,
        ctx: &mut ModelContext<Self>,
    ) {
        if self
            .base_llm_for_terminal_view
            .remove(&terminal_view_id)
            .is_some()
        {
            ctx.emit(LLMPreferencesEvent::UpdatedActiveAgentModeLLM);
        }
    }
}

#[derive(Clone, Debug)]
pub enum LLMPreferencesEvent {
    UpdatedAvailableLLMs,
    UpdatedActiveAgentModeLLM,
    UpdatedActiveCodingLLM,
}

impl Entity for LLMPreferences {
    type Event = LLMPreferencesEvent;
}

impl SingletonEntity for LLMPreferences {}

fn available_llms(id: &str, display_name: &str, vision_supported: bool) -> AvailableLLMs {
    AvailableLLMs {
        default_id: id.into(),
        choices: vec![model_info(id, display_name, vision_supported)],
        preferred_codex_model_id: None,
    }
}

fn model_info(id: &str, display_name: &str, vision_supported: bool) -> LLMInfo {
    LLMInfo {
        display_name: display_name.to_string(),
        base_model_name: display_name.to_string(),
        id: id.into(),
        reasoning_level: None,
        usage_metadata: LLMUsageMetadata {
            request_multiplier: 1,
            credit_multiplier: None,
        },
        description: None,
        disable_reason: Some(DisableReason::Unavailable),
        vision_supported,
        spec: None,
        provider: LLMProvider::Unknown,
        host_configs: HashMap::new(),
        discount_percentage: None,
        context_window: LLMContextWindow::default(),
    }
}
