use std::path::PathBuf;

use pathfinder_geometry::vector::Vector2F;
use warp_util::path::LineAndColumnArg;
use warp_util::standardized_path::StandardizedPath;
use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, ModelHandle, TypedActionView, View, ViewContext, WeakViewHandle};

use crate::code::active_file::ActiveFileModel;
use crate::coding_panel_enablement_state::CodingPanelEnablementState;
use crate::terminal::input::InputDropTargetData;
use crate::terminal::view::TerminalView;
use crate::util::openable_file_type::FileTarget;

/// Stable identifier for an item in the file tree.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FileTreeIdentifier {
    pub root: StandardizedPath,
    pub index: usize,
}

impl std::fmt::Debug for FileTreeIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("FileIdentifier")
    }
}

#[derive(Clone)]
pub enum FileTreeAction {
    ItemClicked {
        id: FileTreeIdentifier,
    },
    SelectPreviousItem,
    SelectNextItem,
    Expand,
    Collapse,
    ExecuteSelectedItem,
    OpenContextMenu {
        position: Vector2F,
        id: FileTreeIdentifier,
    },
    CopyPath {
        id: FileTreeIdentifier,
    },
    CopyRelativePath {
        id: FileTreeIdentifier,
    },
    AttachAsContext {
        id: FileTreeIdentifier,
    },
    OpenInFinder {
        id: FileTreeIdentifier,
    },
    Rename {
        id: FileTreeIdentifier,
    },
    Delete {
        id: FileTreeIdentifier,
    },
    NewFileBelowDirectory {
        id: FileTreeIdentifier,
    },
    OpenInNewPane {
        id: FileTreeIdentifier,
    },
    OpenInNewTab {
        id: FileTreeIdentifier,
    },
    CDToDirectory {
        id: FileTreeIdentifier,
    },
    DismissEditor,
    ItemDroppedOnInput {
        id: FileTreeIdentifier,
        terminal_input_data: InputDropTargetData,
    },
    ItemDroppedOnTerminal {
        id: FileTreeIdentifier,
        terminal_view: WeakViewHandle<TerminalView>,
    },
}

impl std::fmt::Debug for FileTreeAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TreeAction")
    }
}

pub fn init(_app: &mut AppContext) {}

pub struct FileTreeView;

impl FileTreeView {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn set_is_active(&mut self, _is_active: bool, _ctx: &mut ViewContext<Self>) {}

    pub fn set_active_file_model(
        &mut self,
        _active_file_model: ModelHandle<ActiveFileModel>,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(crate) fn set_enablement_state(
        &mut self,
        _enablement: CodingPanelEnablementState,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn set_remote_root_directories(
        &mut self,
        _repos: &[repo_metadata::RemoteRepositoryIdentifier],
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn set_root_directories(&mut self, _paths: Vec<PathBuf>, _ctx: &mut ViewContext<Self>) {}

    pub fn set_has_terminal_session(
        &mut self,
        _has_terminal_session: bool,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn select_first_item_if_no_selection(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn auto_expand_to_most_recent_directory(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn on_left_panel_focused(&mut self, _ctx: &mut ViewContext<Self>) {}
}

pub enum FileTreeEvent {
    AttachAsContext {
        path: PathBuf,
    },
    OpenFile {
        path: PathBuf,
        target: FileTarget,
        line_col: Option<LineAndColumnArg>,
    },
    FileRenamed {
        old_path: PathBuf,
        new_path: PathBuf,
    },
    FileDeleted {
        path: PathBuf,
    },
    CDToDirectory {
        path: PathBuf,
    },
    OpenDirectoryInNewTab {
        path: PathBuf,
    },
}

impl Entity for FileTreeView {
    type Event = FileTreeEvent;
}

impl View for FileTreeView {
    fn ui_name() -> &'static str {
        "FilePicker"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for FileTreeView {
    type Action = FileTreeAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}
