use std::{collections::HashMap, sync::Arc};

use parking_lot::FairMutex;
use warpui::{AppContext, Entity, ModelContext, ModelHandle, SingletonEntity};

use crate::{
    ai::{
        block_context::BlockContext,
        predict::generate_ai_input_suggestions::{
            GenerateAIInputSuggestionsRequest, GenerateAIInputSuggestionsResponseV2,
        },
    },
    ai_assistant::execution_context::WarpAiExecutionContext,
    completer::SessionContext,
    server::server_api::ServerApi,
    terminal::{
        event::UserBlockCompleted,
        input::{CompleterData, IntelligentAutosuggestionResult},
        model::session::Sessions,
        History, HistoryEntry, TerminalModel,
    },
};

pub fn is_next_command_enabled(_app: &AppContext) -> bool {
    false
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct HistoryBasedAutosuggestionState {
    pub history_command_prediction: String,
    pub history_command_prediction_likelihood: f64,
    pub total_history_count: usize,
}

#[derive(Clone, Default, PartialEq)]
pub enum NextCommandSuggestionState {
    #[default]
    None,
    Cycling,
    Ready {
        request: Box<GenerateAIInputSuggestionsRequest>,
        response: GenerateAIInputSuggestionsResponseV2,
        request_duration_ms: i64,
        is_from_ai: bool,
        is_from_cycle: bool,
        history_based_autosuggestion_state: HistoryBasedAutosuggestionState,
    },
}

impl NextCommandSuggestionState {
    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Ready { .. })
    }

    pub fn is_cycling(&self) -> bool {
        matches!(self, Self::Cycling)
    }

    pub fn command_suggestion(&self) -> Option<&str> {
        match self {
            Self::Ready { response, .. } => {
                let command = &response.most_likely_action;
                if command.starts_with('{') {
                    None
                } else {
                    Some(command)
                }
            }
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct ZeroStateSuggestionInfo {
    pub request: Box<GenerateAIInputSuggestionsRequest>,
    pub response: GenerateAIInputSuggestionsResponseV2,
    pub request_duration_ms: i64,
    pub is_from_ai: bool,
    pub history_based_autosuggestion_state: HistoryBasedAutosuggestionState,
}

pub struct NextCommandModel {
    next_command_state: NextCommandSuggestionState,
}

impl Entity for NextCommandModel {
    type Event = NextCommandModelEvent;
}

pub enum NextCommandModelEvent {
    NextCommandSuggestionReady,
}

impl NextCommandModel {
    pub fn new(
        _sessions: ModelHandle<Sessions>,
        _model: Arc<FairMutex<TerminalModel>>,
        _server_api: Arc<ServerApi>,
    ) -> Self {
        Self {
            next_command_state: NextCommandSuggestionState::None,
        }
    }

    #[cfg(feature = "local_fs")]
    pub fn get_similar_history_context(
        _conn: &mut diesel::SqliteConnection,
        _completed_block: &UserBlockCompleted,
        _num_additional_preceding_commands: usize,
    ) -> Vec<crate::ai::predict::generate_ai_input_suggestions::HistoryContext> {
        Vec::new()
    }

    pub fn get_state(&self) -> &NextCommandSuggestionState {
        &self.next_command_state
    }

    pub fn get_zero_state_suggestion_info(&self) -> Option<&ZeroStateSuggestionInfo> {
        None
    }

    pub fn clear_state(&mut self) {
        self.next_command_state = NextCommandSuggestionState::None;
    }

    pub fn abort_inflight_request(&mut self) {}

    pub fn cycle_next_command_suggestion(&mut self, _ctx: &mut ModelContext<Self>) {}

    #[expect(clippy::too_many_arguments)]
    pub fn generate_next_command_suggestion(
        &mut self,
        _block_completed: UserBlockCompleted,
        _context: WarpAiExecutionContext,
        _completer_data: CompleterData,
        _block_context: Option<Box<BlockContext>>,
        _previous_result: Option<IntelligentAutosuggestionResult>,
        _ctx: &mut ModelContext<Self>,
    ) {
        self.clear_state();
    }

    pub fn get_reverse_chronological_potential_autosuggestions(
        prefix: &str,
        completer_data: &CompleterData,
        app: &AppContext,
    ) -> Option<Vec<HistoryEntry>> {
        let session_id = completer_data.active_block_session_id()?;
        let history_entries = History::as_ref(app).commands(session_id)?;
        let working_dir = completer_data
            .active_block_metadata
            .as_ref()
            .and_then(|block_metadata| block_metadata.current_working_directory());
        Some(find_potential_autosuggestions_from_history(
            history_entries.into_iter(),
            prefix,
            working_dir,
        ))
    }

    #[expect(clippy::too_many_arguments)]
    pub fn generate_next_command_suggestion_with_prefix(
        &mut self,
        _prefix: Option<String>,
        _block_completed: UserBlockCompleted,
        _context: WarpAiExecutionContext,
        _completer_data: CompleterData,
        _block_context: Option<Box<BlockContext>>,
        _previous_result: Option<IntelligentAutosuggestionResult>,
        _ctx: &mut ModelContext<Self>,
    ) {
        self.clear_state();
    }
}

impl SingletonEntity for NextCommandModel {}

pub async fn is_command_valid(
    _command: &str,
    _ctx: Option<&SessionContext>,
    _session_env_vars: Option<&HashMap<String, String>>,
) -> bool {
    true
}

fn find_potential_autosuggestions_from_history<'a>(
    history_entries: impl DoubleEndedIterator<Item = &'a HistoryEntry>,
    buffer_text: &str,
    working_dir: Option<&str>,
) -> Vec<HistoryEntry> {
    let mut commands_in_same_dir = vec![];
    let mut commands_in_other_dirs = vec![];
    for entry in history_entries.rev() {
        if !entry.command.starts_with(buffer_text) {
            continue;
        }
        let same_dir = entry
            .pwd
            .as_ref()
            .zip(working_dir)
            .is_some_and(|(pwd, working_dir)| pwd == working_dir);

        if same_dir {
            commands_in_same_dir.push(entry.clone());
        } else {
            commands_in_other_dirs.push(entry.clone());
        }
    }
    commands_in_same_dir.extend(commands_in_other_dirs);
    commands_in_same_dir
}
