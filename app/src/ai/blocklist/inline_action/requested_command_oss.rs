use std::rc::Rc;
use std::sync::{Arc, RwLock};

use lazy_static::lazy_static;
use parking_lot::FairMutex;
use warpui::elements::{ChildView, Element, Empty, MouseStateHandle};
use warpui::keymap::Keystroke;
use warpui::{
    AppContext, Element as _, Entity, EntityId, ModelHandle, TypedActionView, UpdateView, View,
    ViewContext, ViewHandle,
};

use crate::ai::agent::{AIAgentActionId, AIAgentCitation};
use crate::ai::blocklist::block::cli_controller::UserTakeOverReason;
use crate::ai::blocklist::block::model::AIBlockModel;
use crate::ai::blocklist::block::AutonomySettingSpeedbump;
use crate::ai::blocklist::{AIBlock, BlocklistAIActionModel, ClientIdentifiers};
use crate::terminal::TerminalModel;

pub const REQUESTED_COMMAND_BODY_VERTICAL_PADDING: f32 = 16.;
pub const VIEWING_COMMAND_DETAIL_MESSAGE: &str = "";

lazy_static! {
    pub static ref CANCEL_REQUESTED_COMMAND_KEYSTROKE: Keystroke = Keystroke {
        ctrl: true,
        key: "c".to_owned(),
        ..Default::default()
    };
    pub static ref ENTER_ACCEPT_REQUESTED_COMMAND_KEYSTROKE: Keystroke = Keystroke {
        key: "enter".to_owned(),
        ..Default::default()
    };
}

pub fn init(_app: &mut AppContext) {}

#[derive(Clone, PartialEq, Eq)]
pub enum RequestedActionViewType {
    Command,
    McpTool,
}

impl std::fmt::Debug for RequestedActionViewType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RequestedAction")
    }
}

impl RequestedActionViewType {
    fn is_requested_command(&self) -> bool {
        matches!(self, RequestedActionViewType::Command)
    }
}

#[derive(Clone)]
pub enum RequestedCommandViewEvent {
    Accepted,
    EnableAutoexecuteMode,
    Rejected,
    UpdatedExpansionState { is_expanded: bool },
    TextSelected,
    CopiedEmptyText,
    EditorFocused,
    OpenActiveAgentProfileEditor,
}

impl std::fmt::Debug for RequestedCommandViewEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RequestedEvent")
    }
}

#[derive(Clone)]
pub enum RequestedCommandViewAction {
    Accept,
    AcceptAndAutoExecute,
    ToggleAcceptMenu,
    Reject,
    OpenEditMode,
    CloseEditMode,
    FocusEditor,
    ToggleExpanded,
    OpenActiveAgentProfileEditor,
    SelectText,
}

impl std::fmt::Debug for RequestedCommandViewAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RequestedAction")
    }
}

pub struct RequestedCommandView {
    command_text: String,
    action_type: RequestedActionViewType,
    is_header_expanded: bool,
    copied_from_citation: Option<AIAgentCitation>,
    derived_from_citations: Vec<AIAgentCitation>,
    selected_text: Arc<RwLock<Option<String>>>,
}

impl RequestedCommandView {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        _action_id: AIAgentActionId,
        _client_ids: ClientIdentifiers,
        action_type: RequestedActionViewType,
        _block_model: Rc<dyn AIBlockModel<View = AIBlock>>,
        _action_model: &ModelHandle<BlocklistAIActionModel>,
        _terminal_model: Arc<FairMutex<TerminalModel>>,
        _autonomy_setting_speedbump: AutonomySettingSpeedbump,
        _manage_autonomy_settings_link_handle: MouseStateHandle,
        _ai_block_view_id: EntityId,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            command_text: String::new(),
            action_type,
            is_header_expanded: false,
            copied_from_citation: None,
            derived_from_citations: Vec::new(),
            selected_text: Arc::new(RwLock::new(None)),
        }
    }

    pub fn is_header_expanded(&self) -> bool {
        self.is_header_expanded
    }

    pub fn command_text(&self) -> &str {
        &self.command_text
    }

    pub fn commit_and_get_command_text(&mut self, _ctx: &mut ViewContext<Self>) -> String {
        self.command_text.clone()
    }

    pub fn copied_from_citation(&self) -> Option<&AIAgentCitation> {
        self.copied_from_citation.as_ref()
    }

    pub fn update_copied_from_citation(&mut self, citation: &AIAgentCitation) {
        self.copied_from_citation = Some(citation.clone());
    }

    pub fn update_derived_from_citations(&mut self, citations: &[AIAgentCitation]) {
        self.derived_from_citations = citations.to_vec();
    }

    pub fn set_autonomy_setting_speedbump(
        &mut self,
        _speedbump: AutonomySettingSpeedbump,
        ctx: &mut ViewContext<Self>,
    ) {
        ctx.notify();
    }

    pub fn apply_streamed_update(&mut self, command: &str, _ctx: &mut ViewContext<Self>) {
        self.command_text = command.to_string();
    }

    pub fn selected_text(&self, _ctx: &AppContext) -> Option<String> {
        self.selected_text
            .read()
            .ok()
            .and_then(|selected_text| selected_text.clone())
    }

    pub fn clear_selection(&mut self, _ctx: &mut ViewContext<Self>) {
        if let Ok(mut selected_text) = self.selected_text.write() {
            *selected_text = None;
        }
    }

    fn set_is_header_expanded(&mut self, is_expanded: bool, ctx: &mut ViewContext<Self>) {
        if self.is_header_expanded != is_expanded {
            self.is_header_expanded = is_expanded;
            ctx.emit(RequestedCommandViewEvent::UpdatedExpansionState { is_expanded });
            ctx.notify();
        }
    }
}

impl Entity for RequestedCommandView {
    type Event = RequestedCommandViewEvent;
}

impl View for RequestedCommandView {
    fn ui_name() -> &'static str {
        "RequestedView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for RequestedCommandView {
    type Action = RequestedCommandViewAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            RequestedCommandViewAction::Accept => {
                ctx.emit(RequestedCommandViewEvent::Accepted);
            }
            RequestedCommandViewAction::AcceptAndAutoExecute => {
                ctx.emit(RequestedCommandViewEvent::Accepted);
                ctx.emit(RequestedCommandViewEvent::EnableAutoexecuteMode);
            }
            RequestedCommandViewAction::Reject => ctx.emit(RequestedCommandViewEvent::Rejected),
            RequestedCommandViewAction::ToggleExpanded => {
                self.set_is_header_expanded(!self.is_header_expanded, ctx);
            }
            RequestedCommandViewAction::OpenActiveAgentProfileEditor => {
                ctx.emit(RequestedCommandViewEvent::OpenActiveAgentProfileEditor)
            }
            RequestedCommandViewAction::SelectText => {
                ctx.emit(RequestedCommandViewEvent::TextSelected);
            }
            RequestedCommandViewAction::OpenEditMode
            | RequestedCommandViewAction::CloseEditMode
            | RequestedCommandViewAction::FocusEditor
            | RequestedCommandViewAction::ToggleAcceptMenu => {}
        }
    }
}

pub(crate) fn header_message_for_user_take_over_reason(
    _reason: &UserTakeOverReason,
) -> &'static str {
    ""
}

pub struct RequestedCommand {
    pub view: ViewHandle<RequestedCommandView>,
}

impl RequestedCommand {
    pub fn render(&self) -> Box<dyn Element> {
        ChildView::new(&self.view).finish()
    }

    pub fn force_expand(&self, ctx: &mut impl UpdateView) {
        self.view.update(ctx, |command, ctx| {
            command.set_is_header_expanded(true, ctx);
        })
    }

    pub fn force_collapse(&self, ctx: &mut impl UpdateView) {
        self.view.update(ctx, |command, ctx| {
            command.set_is_header_expanded(false, ctx);
        })
    }
}

pub fn format_command_text(text: &str) -> String {
    if let Some(newline_pos) = text.find('\n') {
        let first_line = &text[..newline_pos];
        if text[newline_pos..].trim().is_empty() {
            first_line.to_string()
        } else {
            format!("{first_line}...")
        }
    } else {
        text.to_string()
    }
}
