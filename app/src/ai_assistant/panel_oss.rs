use std::sync::Arc;

use warpui::elements::{Element, Empty, HyperlinkUrl};
use warpui::{AppContext, Entity, TypedActionView, View, ViewContext};

use crate::server::server_api::ai::AIClient;
use crate::server::server_api::ServerApi;

use super::AskAIType;

pub enum AIAssistantPanelEvent {
    ClosePanel,
    PasteInTerminalInput(Arc<String>),
    FocusTerminalInput,
    OpenWorkflowModalWithCommand(String),
}

pub struct AIAssistantPanelView;

#[derive(Clone)]
pub enum AIAssistantAction {
    ClosePanel,
    ResetContext,
    CopyTranscript,
    PreparedPrompt(&'static str),
    ClickedUrl(HyperlinkUrl),
    CopyAnswerToClipboard(Arc<String>),
    FocusTerminalInput,
    FocusEditor,
}

impl std::fmt::Debug for AIAssistantAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AssistantAction")
    }
}

pub fn init(_app: &mut AppContext) {}

impl AIAssistantPanelView {
    pub fn new(
        _server_api: Arc<ServerApi>,
        _ai_client: Arc<dyn AIClient>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn ask_ai(&mut self, _ask_type: &AskAIType, _ctx: &mut ViewContext<Self>) {}

    #[cfg(feature = "integration_tests")]
    pub fn editor(&self) -> &warpui::ViewHandle<crate::editor::EditorView> {
        unimplemented!("panel is unavailable in OSS release builds")
    }
}

impl Entity for AIAssistantPanelView {
    type Event = AIAssistantPanelEvent;
}

impl TypedActionView for AIAssistantPanelView {
    type Action = AIAssistantAction;

    fn handle_action(&mut self, action: &AIAssistantAction, ctx: &mut ViewContext<Self>) {
        match action {
            AIAssistantAction::ClosePanel => ctx.emit(AIAssistantPanelEvent::ClosePanel),
            AIAssistantAction::FocusTerminalInput => {
                ctx.emit(AIAssistantPanelEvent::FocusTerminalInput)
            }
            _ => {}
        }
    }
}

impl View for AIAssistantPanelView {
    fn ui_name() -> &'static str {
        "AssistantPanel"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
