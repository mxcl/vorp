pub mod editor {
    use warpui::{
        AppContext, Element, Entity, TypedActionView, View, ViewContext, elements::Empty,
    };

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum AgentToolbarEditorMode {
        #[default]
        AgentView,
        CLIAgent,
    }

    pub enum AgentToolbarEditorEvent {
        Close,
    }

    pub struct AgentToolbarEditorModal;

    pub struct AgentToolbarInlineEditor;

    #[derive(Clone, Copy, Debug)]
    pub enum AgentToolbarEditorAction {
        Cancel,
    }

    #[derive(Clone, Copy, Debug)]
    pub enum AgentToolbarInlineEditorAction {}

    impl AgentToolbarInlineEditor {
        pub fn new(_mode: AgentToolbarEditorMode, _ctx: &mut ViewContext<Self>) -> Self {
            Self
        }
    }

    impl Entity for AgentToolbarInlineEditor {
        type Event = ();
    }

    impl TypedActionView for AgentToolbarInlineEditor {
        type Action = AgentToolbarInlineEditorAction;

        fn handle_action(&mut self, action: &Self::Action, _ctx: &mut ViewContext<Self>) {
            match *action {}
        }
    }

    impl View for AgentToolbarInlineEditor {
        fn ui_name() -> &'static str {
            "AgentToolbarInlineEditor"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }

    pub fn init(_app: &mut AppContext) {}

    impl AgentToolbarEditorModal {
        pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
            Self
        }

        pub fn open(&mut self, _mode: AgentToolbarEditorMode, _ctx: &mut ViewContext<Self>) {}
    }

    impl Entity for AgentToolbarEditorModal {
        type Event = AgentToolbarEditorEvent;
    }

    impl TypedActionView for AgentToolbarEditorModal {
        type Action = AgentToolbarEditorAction;

        fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
            match action {
                AgentToolbarEditorAction::Cancel => ctx.emit(AgentToolbarEditorEvent::Close),
            }
        }
    }

    impl View for AgentToolbarEditorModal {
        fn ui_name() -> &'static str {
            "AgentToolbarEditorModal"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }
}

#[path = "agent_input_footer/toolbar_item.rs"]
pub mod toolbar_item;

use std::sync::Arc;

use parking_lot::FairMutex;
use pathfinder_color::ColorU;
use pathfinder_geometry::vector::Vector2F;
use warp_core::ui::{appearance::Appearance, theme::Fill};
use warpui::{
    AppContext, Element, Entity, EntityId, ModelHandle, TypedActionView, View, ViewContext,
    elements::Empty,
};

use crate::{
    ai::{
        blocklist::{BlocklistAIInputModel, prompt::prompt_alert::PromptAlertEvent},
        document::ai_document_model::{AIDocumentId, AIDocumentVersion},
    },
    completer::SessionContext,
    context_chips::{display_chip::DisplayChipConfig, prompt_type::PromptType},
    settings_view::SettingsSection,
    terminal::{
        CLIAgent, TerminalModel,
        input::{MenuPositioningProvider, models::InlineModelSelectorTab},
        view::ambient_agent::AmbientAgentViewModel,
    },
    view_components::action_button::ActionButtonTheme,
};

#[cfg(not(target_family = "wasm"))]
use crate::terminal::cli_agent_sessions::plugin_manager::PluginModalKind;

#[cfg(feature = "local_fs")]
use crate::ai::cloud_environments::CloudAmbientAgentEnvironment;

pub struct AgentInputFooter;

impl AgentInputFooter {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        _menu_positioning_provider: Arc<dyn MenuPositioningProvider>,
        _terminal_view_id: EntityId,
        _ai_input_model: ModelHandle<BlocklistAIInputModel>,
        _terminal_model: Arc<FairMutex<TerminalModel>>,
        _ambient_agent_view_model: Option<ModelHandle<AmbientAgentViewModel>>,
        _prompt: ModelHandle<PromptType>,
        _display_chip_config: DisplayChipConfig,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn set_current_repo_path(
        &mut self,
        _repo_path: Option<std::path::PathBuf>,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn is_v2_model_selector_open(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn open_v2_model_selector(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn is_v2_environment_selector_open(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn open_v2_environment_selector(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn update_session_context(
        &mut self,
        _session_context: Option<SessionContext>,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn has_open_chip_menu(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn is_model_selector_open(&self, _app: &AppContext) -> bool {
        false
    }

    pub fn set_voice_is_active(&mut self, _is_active: bool, _ctx: &mut ViewContext<Self>) {}

    #[cfg(feature = "voice_input")]
    pub fn toggle_cli_voice_input(
        &mut self,
        _source: &voice_input::VoiceInputToggledFrom,
        _ctx: &mut ViewContext<Self>,
    ) {
    }
}

impl View for AgentInputFooter {
    fn ui_name() -> &'static str {
        "AgentInputFooter"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

#[derive(Debug)]
pub enum AgentInputFooterAction {}

impl TypedActionView for AgentInputFooter {
    type Action = AgentInputFooterAction;

    fn handle_action(&mut self, action: &Self::Action, _ctx: &mut ViewContext<Self>) {
        match *action {}
    }
}

pub enum AgentInputFooterEvent {
    #[cfg(feature = "voice_input")]
    ToggleVoiceInput(voice_input::VoiceInputToggledFrom),
    SelectFile,
    WriteToPty(String),
    InsertIntoCLIRichInput(String),
    ToggleCodeReviewPane(CLIAgent),
    ToggleFileExplorer(CLIAgent),
    StartRemoteControl,
    StopRemoteControl,
    OpenRichInput,
    HideRichInput,
    ToggledChipMenu {
        open: bool,
    },
    TryExecuteChipCommand(String),
    PromptAlert(PromptAlertEvent),
    ModelSelectorOpened,
    ModelSelectorClosed,
    EnvironmentSelectorClosed,
    ToggleInlineModelSelector {
        initial_tab: InlineModelSelectorTab,
    },
    OpenSettings(SettingsSection),
    OpenCodeReview,
    OpenAIDocument {
        document_id: AIDocumentId,
        document_version: AIDocumentVersion,
    },
    ShowContextMenu {
        position: Vector2F,
    },
    OpenEnvironmentManagementPane,
    #[cfg(not(target_family = "wasm"))]
    OpenPluginInstructionsPane(CLIAgent, PluginModalKind),
    OpenHandoffPane {
        initial_prompt: Option<String>,
    },
}

impl Entity for AgentInputFooter {
    type Event = AgentInputFooterEvent;
}

#[cfg(feature = "local_fs")]
pub(crate) fn sort_environments_by_recency(environments: &mut [CloudAmbientAgentEnvironment]) {
    environments.sort_by(|a, b| {
        b.metadata
            .last_task_run_ts
            .cmp(&a.metadata.last_task_run_ts)
            .then_with(|| {
                a.model()
                    .string_model
                    .name
                    .to_lowercase()
                    .cmp(&b.model().string_model.name.to_lowercase())
            })
    });
}

pub(crate) struct AgentInputButtonTheme;

impl ActionButtonTheme for AgentInputButtonTheme {
    fn background(&self, hovered: bool, appearance: &Appearance) -> Option<Fill> {
        let theme = appearance.theme();
        Some(if hovered {
            theme.surface_2()
        } else {
            theme.surface_1()
        })
    }

    fn text_color(&self, _: bool, background: Option<Fill>, appearance: &Appearance) -> ColorU {
        appearance
            .theme()
            .main_text_color(background.unwrap_or(appearance.theme().background()))
            .into_solid()
    }

    fn keyboard_shortcut_background(&self, appearance: &Appearance) -> Option<ColorU> {
        Some(appearance.theme().surface_overlay_2().into_solid())
    }
}

pub(crate) struct ActiveMicButtonTheme;

impl ActionButtonTheme for ActiveMicButtonTheme {
    fn background(&self, hovered: bool, appearance: &Appearance) -> Option<Fill> {
        AgentInputButtonTheme.background(hovered, appearance)
    }

    fn text_color(
        &self,
        hovered: bool,
        background: Option<Fill>,
        appearance: &Appearance,
    ) -> ColorU {
        AgentInputButtonTheme.text_color(hovered, background, appearance)
    }

    fn keyboard_shortcut_background(&self, appearance: &Appearance) -> Option<ColorU> {
        AgentInputButtonTheme.keyboard_shortcut_background(appearance)
    }
}
