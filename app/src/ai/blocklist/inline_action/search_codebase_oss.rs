use std::ops::Range;
use std::sync::{Arc, RwLock};

use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, ModelHandle, TypedActionView, View, ViewContext};

use crate::ai::agent::FileContext;
use crate::ai::blocklist::action_model::AIActionStatus;
use crate::ai::blocklist::block::find::FindState;
use crate::ai::blocklist::TextLocation;
use crate::terminal::find::TerminalFindModel;
use crate::terminal::view::RichContentLink;
use crate::terminal::ShellLaunchData;

pub enum SearchCodebaseViewEvent {
    OpenLinkTooltip {
        rich_content_link: RichContentLink,
    },
    #[cfg(feature = "local_fs")]
    OpenDetectedFilePath {
        absolute_path: std::path::PathBuf,
        line_and_column_num: Option<warp_util::path::LineAndColumnArg>,
    },
    TextSelected,
}

#[derive(Clone)]
pub enum SearchCodebaseViewAction {
    ToggleExpanded,
    OpenLink {
        link_range: Range<usize>,
        location: TextLocation,
    },
    ChangedHoverOnLink {
        link_range: Range<usize>,
        location: TextLocation,
        is_hovering: bool,
    },
    OpenLinkTooltip {
        location: TextLocation,
        link_range: Range<usize>,
    },
    SelectText,
}

impl std::fmt::Debug for SearchCodebaseViewAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SearchAction")
    }
}

pub struct SearchCodebaseView {
    selected_text: Arc<RwLock<Option<String>>>,
}

impl SearchCodebaseView {
    pub fn new(
        _find_model: ModelHandle<TerminalFindModel>,
        _file_contexts: Vec<FileContext>,
        _search_query: String,
        _repo_path: Option<String>,
        _shell_launch_data: &Option<ShellLaunchData>,
        _current_working_directory: &Option<String>,
        _action_index: usize,
    ) -> Self {
        Self {
            selected_text: Arc::new(RwLock::new(None)),
        }
    }

    pub fn update_status(&mut self, _status: Option<AIActionStatus>) {}

    pub fn update_render_read_file_args(
        &mut self,
        _find_state: &FindState,
        _file_contexts: Vec<FileContext>,
        _status: Option<AIActionStatus>,
    ) {
    }

    pub fn clear_link_tooltip(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn clear_selection(&mut self, _ctx: &mut ViewContext<Self>) {
        if let Ok(mut selected_text) = self.selected_text.write() {
            *selected_text = None;
        }
    }

    pub fn selected_text(&self, _ctx: &AppContext) -> Option<String> {
        self.selected_text
            .read()
            .ok()
            .and_then(|selected_text| selected_text.clone())
    }
}

impl Entity for SearchCodebaseView {
    type Event = SearchCodebaseViewEvent;
}

impl TypedActionView for SearchCodebaseView {
    type Action = SearchCodebaseViewAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        if matches!(action, SearchCodebaseViewAction::SelectText) {
            ctx.emit(SearchCodebaseViewEvent::TextSelected);
        }
    }
}

impl View for SearchCodebaseView {
    fn ui_name() -> &'static str {
        "SearchView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
