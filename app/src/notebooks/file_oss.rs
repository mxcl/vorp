use std::{path::PathBuf, sync::Arc};

use crate::{
    pane_group::{
        focus_state::PaneFocusHandle,
        pane::view::{HeaderContent, HeaderRenderContext},
        BackingView, PaneConfiguration, PaneEvent,
    },
    terminal::model::session::Session,
    workflows::{WorkflowSource, WorkflowType},
};
use warpui::{
    elements::Empty, AppContext, Element, Entity, ModelHandle, TypedActionView, View, ViewContext,
};

#[cfg(feature = "local_fs")]
use crate::code::editor_management::CodeSource;
#[cfg(feature = "local_fs")]
use crate::util::openable_file_type::FileTarget;

pub use crate::util::openable_file_type::is_markdown_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkdownDisplayMode {
    Rendered,
    Raw,
}

pub struct FileNotebookView {
    pane_configuration: ModelHandle<PaneConfiguration>,
    local_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum FileNotebookEvent {
    RunWorkflow {
        workflow: Arc<WorkflowType>,
        source: WorkflowSource,
    },
    TitleUpdated,
    FileLoaded,
    Pane(PaneEvent),
    #[cfg(feature = "local_fs")]
    OpenFileWithTarget {
        path: PathBuf,
        target: FileTarget,
        line_col: Option<warp_util::path::LineAndColumnArg>,
    },
}

impl FileNotebookView {
    pub fn new(ctx: &mut ViewContext<Self>) -> Self {
        Self {
            pane_configuration: ctx.add_model(|_| PaneConfiguration::new("")),
            local_path: None,
        }
    }

    #[cfg(feature = "local_fs")]
    pub fn set_code_source(&mut self, _code_source: Option<CodeSource>) {}

    pub fn open_local(
        &mut self,
        path: PathBuf,
        _target_session: Option<Arc<Session>>,
        ctx: &mut ViewContext<Self>,
    ) {
        self.local_path = Some(path);
        ctx.emit(FileNotebookEvent::FileLoaded);
    }

    pub fn open_static(&mut self, title: &str, _content: &str, ctx: &mut ViewContext<Self>) {
        self.pane_configuration
            .update(ctx, |config, ctx| config.set_title(title.to_string(), ctx));
    }

    pub fn pane_configuration(&self) -> ModelHandle<PaneConfiguration> {
        self.pane_configuration.clone()
    }

    pub fn local_path(&self) -> Option<PathBuf> {
        self.local_path.clone()
    }

    pub fn focus(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.focus_self();
    }

    pub fn selected_text(&self, _ctx: &AppContext) -> Option<String> {
        None
    }

    pub fn set_focus_handle(
        &mut self,
        _focus_handle: PaneFocusHandle,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn set_markdown_display_mode(
        &mut self,
        _mode: MarkdownDisplayMode,
        _ctx: &mut ViewContext<Self>,
    ) {
    }
}

impl Entity for FileNotebookView {
    type Event = FileNotebookEvent;
}

impl View for FileNotebookView {
    fn ui_name() -> &'static str {
        "FileNotebookView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for FileNotebookView {
    type Action = ();

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl BackingView for FileNotebookView {
    type PaneHeaderOverflowMenuAction = ();
    type CustomAction = ();
    type AssociatedData = ();

    fn handle_pane_header_overflow_menu_action(
        &mut self,
        _action: &Self::PaneHeaderOverflowMenuAction,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    fn close(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(FileNotebookEvent::Pane(PaneEvent::Close));
    }

    fn focus_contents(&mut self, ctx: &mut ViewContext<Self>) {
        self.focus(ctx);
    }

    fn render_header_content(
        &self,
        _ctx: &HeaderRenderContext<'_>,
        app: &AppContext,
    ) -> HeaderContent {
        HeaderContent::simple(self.pane_configuration.as_ref(app).title())
    }

    fn set_focus_handle(&mut self, focus_handle: PaneFocusHandle, ctx: &mut ViewContext<Self>) {
        self.set_focus_handle(focus_handle, ctx);
    }
}
