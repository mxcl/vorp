use std::path::{Path, PathBuf};

use pathfinder_geometry::rect::RectF;
use warp_util::path::LineAndColumnArg;
use warpui::{
    elements::Empty, AppContext, Element, Entity, ModelHandle, TypedActionView, View, ViewContext,
};

use crate::menu::MenuItem;
use crate::pane_group::{
    focus_state::PaneFocusHandle,
    pane::view::{HeaderContent, HeaderRenderContext},
    BackingView, CodePane, PaneConfiguration, PaneEvent,
};

use super::editor_management::CodeSource;

pub use crate::util::openable_file_type::is_binary_file;

pub const SAVE_FILE_BINDING_NAME: &str = "pane:save";
pub const SAVE_FILE_BINDING_DESCRIPTION: &str = "Save file";

pub fn init(_app: &mut AppContext) {}

/// Determines the `SavePosition` ID for a draggable tab based on its index.
pub fn tab_position_id(index: usize) -> String {
    format!("file_tab_position_{index}")
}

#[derive(Clone)]
pub enum CodeViewAction {
    SaveFile,
    SaveFileAs,
    AcceptPendingDiffsAndSave,
    RejectPendingDiffs,
    SetCurrentTabIndex {
        index: usize,
    },
    RemoveTabAtIndex {
        index: usize,
    },
    CloseAll,
    CloseSaved,
    ToggleMaximized,
    #[cfg(feature = "local_fs")]
    CopyFilePath,
    #[cfg(feature = "local_fs")]
    RenderMarkdown,
    DragOverIndex {
        target: usize,
        drag_position: RectF,
    },
    DropAtIndex {
        origin: usize,
        target: usize,
        drag_position: RectF,
    },
    ClearEditorTabGroupDragPositions,
    ClearWorkspaceTabGroupDragPositions,
}

impl std::fmt::Debug for CodeViewAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PaneAction")
    }
}

#[derive(Clone)]
pub enum CodeViewEvent {
    Pane(PaneEvent),
    TabChanged {
        file_path: Option<PathBuf>,
        tab_index: usize,
    },
    FileOpened {
        file_path: PathBuf,
        tab_index: usize,
    },
    RunTabConfigSkill {
        path: PathBuf,
    },
    OpenLspLogs {
        log_path: PathBuf,
    },
}

impl std::fmt::Debug for CodeViewEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PaneEvent")
    }
}

#[derive(Clone)]
pub struct TabData {
    path: Option<PathBuf>,
}

impl TabData {
    pub fn path(&self) -> Option<PathBuf> {
        self.path.clone()
    }
}

#[derive(Clone)]
pub enum PendingSaveIntent {
    Save,
    Discard,
    Cancel,
}

impl std::fmt::Debug for PendingSaveIntent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SaveIntent")
    }
}

pub struct CodeView {
    tab_group: Vec<TabData>,
    active_tab_index: usize,
    pane_configuration: ModelHandle<PaneConfiguration>,
    source: CodeSource,
    focus_handle: Option<PaneFocusHandle>,
}

impl CodeView {
    pub fn new(
        source: CodeSource,
        line_col: Option<LineAndColumnArg>,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        let mut view = Self::new_internal(source, ctx);
        view.open_or_focus_existing(view.source.path(), line_col, ctx);
        view
    }

    pub fn restore(
        tabs: &[crate::app_state::CodePaneTabSnapshot],
        active_tab_index: usize,
        source: CodeSource,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        let mut view = Self::new_internal(source, ctx);
        view.tab_group = tabs
            .iter()
            .map(|tab| TabData {
                path: tab.path.clone(),
            })
            .collect();
        view.active_tab_index = if view.tab_group.is_empty() {
            0
        } else {
            active_tab_index.min(view.tab_group.len() - 1)
        };
        view
    }

    pub fn new_preview(source: CodeSource, ctx: &mut ViewContext<Self>) -> Self {
        let mut view = Self::new_internal(source, ctx);
        if let Some(path) = view.source.path() {
            view.open_in_preview_or_promote(path, ctx);
        }
        view
    }

    fn new_internal(source: CodeSource, ctx: &mut ViewContext<Self>) -> Self {
        Self {
            tab_group: Vec::new(),
            active_tab_index: 0,
            pane_configuration: ctx.add_model(|_ctx| PaneConfiguration::new("")),
            source,
            focus_handle: None,
        }
    }

    pub fn tab_at(&self, index: usize) -> Option<&TabData> {
        self.tab_group.get(index)
    }

    pub fn active_tab_index(&self) -> usize {
        self.active_tab_index
    }

    pub fn source(&self) -> &CodeSource {
        &self.source
    }

    pub fn selected_text(&self, _ctx: &AppContext) -> Option<String> {
        None
    }

    pub fn local_path(&self, _ctx: &AppContext) -> Option<PathBuf> {
        self.tab_at(self.active_tab_index)
            .and_then(|tab| tab.path())
            .or_else(|| self.source.path())
    }

    pub fn pane_configuration(&self) -> ModelHandle<PaneConfiguration> {
        self.pane_configuration.clone()
    }

    pub fn focus(&self, ctx: &mut ViewContext<Self>) {
        ctx.focus_self();
    }

    pub fn open_in_preview_or_promote(&mut self, path: PathBuf, ctx: &mut ViewContext<Self>) {
        self.open_or_focus_existing(Some(path), None, ctx);
    }

    pub fn open_in_preview_or_promote_and_jump(
        &mut self,
        path: PathBuf,
        line_col: Option<LineAndColumnArg>,
        ctx: &mut ViewContext<Self>,
    ) {
        self.open_or_focus_existing(Some(path), line_col, ctx);
    }

    pub fn open_or_focus_existing(
        &mut self,
        path: Option<PathBuf>,
        _line_col: Option<LineAndColumnArg>,
        ctx: &mut ViewContext<Self>,
    ) {
        let Some(path) = path else {
            return;
        };

        if let Some(index) = self
            .tab_group
            .iter()
            .position(|tab| tab.path.as_ref() == Some(&path))
        {
            self.set_active_tab_index(index, ctx);
            return;
        }

        self.tab_group.push(TabData {
            path: Some(path.clone()),
        });
        let index = self.tab_group.len() - 1;
        self.set_active_tab_index(index, ctx);
        ctx.emit(CodeViewEvent::FileOpened {
            file_path: path,
            tab_index: index,
        });
    }

    pub fn tab_count(&self) -> usize {
        self.tab_group.len()
    }

    pub fn contains_unsaved_changes(&self, _ctx: &AppContext) -> bool {
        false
    }

    pub fn active_tab_has_unsaved_changes(&self, _ctx: &AppContext) -> bool {
        false
    }

    pub fn cleanup_all_tabs(&mut self, ctx: &mut ViewContext<Self>) {
        self.tab_group.clear();
        self.active_tab_index = 0;
        ctx.emit(CodeViewEvent::Pane(PaneEvent::AppStateChanged));
        ctx.notify();
    }

    pub fn close_overlays(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn remove_tab_for_move(
        &mut self,
        index: usize,
        ctx: &mut ViewContext<Self>,
    ) -> Option<CodePane> {
        let path = self.tab_at(index).and_then(|tab| tab.path())?;
        self.remove_tab_at_index(index, ctx);
        Some(CodePane::new(
            CodeSource::Link {
                path,
                range_start: None,
                range_end: None,
            },
            None,
            ctx,
        ))
    }

    pub fn set_active_tab_index(&mut self, index: usize, ctx: &mut ViewContext<Self>) {
        if self.tab_group.is_empty() {
            self.active_tab_index = 0;
            return;
        }
        self.active_tab_index = index.min(self.tab_group.len() - 1);
        let file_path = self
            .tab_at(self.active_tab_index)
            .and_then(|tab| tab.path());
        ctx.emit(CodeViewEvent::TabChanged {
            file_path,
            tab_index: self.active_tab_index,
        });
        ctx.notify();
    }

    pub fn close_tabs_with_path(&mut self, file_path: &Path, ctx: &mut ViewContext<Self>) {
        self.tab_group
            .retain(|tab| tab.path.as_deref() != Some(file_path));
        self.set_active_tab_index(self.active_tab_index, ctx);
    }

    pub fn rename_tabs_with_path(
        &mut self,
        old_path: &Path,
        new_path: &Path,
        ctx: &mut ViewContext<Self>,
    ) {
        for tab in &mut self.tab_group {
            if tab.path.as_deref() == Some(old_path) {
                tab.path = Some(new_path.to_path_buf());
            }
        }
        self.set_active_tab_index(self.active_tab_index, ctx);
    }

    pub fn merge_tabs(&mut self, source_code_view: &CodeView, ctx: &mut ViewContext<Self>) {
        for tab in &source_code_view.tab_group {
            let Some(path) = tab.path.clone() else {
                continue;
            };
            if self
                .tab_group
                .iter()
                .all(|existing| existing.path.as_ref() != Some(&path))
            {
                self.tab_group.push(TabData { path: Some(path) });
            }
        }
        self.set_active_tab_index(self.active_tab_index, ctx);
    }

    fn remove_tab_at_index(&mut self, index: usize, ctx: &mut ViewContext<Self>) {
        if index < self.tab_group.len() {
            self.tab_group.remove(index);
        }
        self.set_active_tab_index(self.active_tab_index.saturating_sub(1), ctx);
    }
}

impl Entity for CodeView {
    type Event = CodeViewEvent;
}

impl View for CodeView {
    fn ui_name() -> &'static str {
        "PaneView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for CodeView {
    type Action = CodeViewAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            CodeViewAction::RemoveTabAtIndex { index } => self.remove_tab_at_index(*index, ctx),
            CodeViewAction::CloseAll => self.cleanup_all_tabs(ctx),
            CodeViewAction::ToggleMaximized => {
                ctx.emit(CodeViewEvent::Pane(PaneEvent::ToggleMaximized))
            }
            CodeViewAction::ClearWorkspaceTabGroupDragPositions => {
                ctx.emit(CodeViewEvent::Pane(PaneEvent::ClearHoveredTabIndex))
            }
            _ => {}
        }
    }
}

impl BackingView for CodeView {
    type PaneHeaderOverflowMenuAction = CodeViewAction;
    type CustomAction = CodeViewAction;
    type AssociatedData = ();

    fn pane_header_overflow_menu_items(&self, _ctx: &AppContext) -> Vec<MenuItem<CodeViewAction>> {
        vec![]
    }

    fn handle_pane_header_overflow_menu_action(
        &mut self,
        action: &Self::PaneHeaderOverflowMenuAction,
        ctx: &mut ViewContext<Self>,
    ) {
        self.handle_action(action, ctx);
    }

    fn close(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(CodeViewEvent::Pane(PaneEvent::Close));
    }

    fn focus_contents(&mut self, ctx: &mut ViewContext<Self>) {
        self.focus(ctx);
    }

    fn handle_custom_action(
        &mut self,
        custom_action: &Self::CustomAction,
        ctx: &mut ViewContext<Self>,
    ) {
        self.handle_action(custom_action, ctx);
    }

    fn render_header_content(
        &self,
        _ctx: &HeaderRenderContext<'_>,
        app: &AppContext,
    ) -> HeaderContent {
        HeaderContent::simple(self.pane_configuration.as_ref(app).title())
    }

    fn set_focus_handle(&mut self, focus_handle: PaneFocusHandle, _ctx: &mut ViewContext<Self>) {
        self.focus_handle = Some(focus_handle);
    }
}
