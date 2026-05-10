use crate::appearance::Appearance;
use pathfinder_color::ColorU;
use warp_core::ui::theme::AnsiColorIdentifier;

pub mod line {
    use std::ops::Range;
    use warp_editor::render::model::LineCount;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum EditorLineLocation {
        Current {
            line_number: LineCount,
            line_range: Range<LineCount>,
        },
        Removed {
            line_number: LineCount,
            line_range: Range<LineCount>,
            index: usize,
        },
        Collapsed {
            line_range: Range<LineCount>,
        },
    }

    impl EditorLineLocation {
        pub fn line_number(&self) -> Option<LineCount> {
            match self {
                Self::Current { line_number, .. } | Self::Removed { line_number, .. } => {
                    Some(*line_number)
                }
                Self::Collapsed { .. } => None,
            }
        }
    }
}

pub mod model {
    #[derive(Debug, Clone)]
    pub struct StableEditorLine;
}

pub mod scroll {
    #[derive(Debug, Clone)]
    pub enum ScrollPosition {
        None,
        FocusedDiffHunk,
        LineAndColumn(warp_util::path::LineAndColumnArg),
    }

    #[derive(Debug, Clone)]
    pub struct ScrollTrigger;

    impl ScrollTrigger {
        pub fn new<T>(_position: ScrollPosition, _version: T) -> Self {
            Self
        }
    }
}

pub mod diff {
    use std::ops::Range;

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum ChangeType {
        Replacement { replaced_range: Range<usize> },
        Addition,
        Deletion,
    }
}

pub mod view {
    use warp_core::platform::SessionPlatform;
    use warp_editor::content::buffer::Buffer;
    use warpui::elements::Empty;
    use warpui::{AppContext, Element, Entity, TypedActionView, View, ViewContext};

    #[derive(Clone)]
    pub enum CodeEditorEvent {
        Focused,
        SelectionChanged,
        ContentChanged { origin: () },
        DiffUpdated,
        UnifiedDiffComputed(String),
        DiffHunkContextAdded { line_range: std::ops::Range<usize> },
        DiffReverted,
        HiddenSectionExpanded,
        CopiedEmptyText,
        VimEscapeInNormalMode,
        VimGotoDefinition,
        VimFindReferences,
        VimShowHover,
        MouseHovered,
        RequestOpenComment(uuid::Uuid),
    }

    #[derive(Debug, Clone)]
    pub struct CodeEditorRenderOptions;

    impl CodeEditorRenderOptions {
        pub fn new<T>(_vertical_expansion_behavior: T) -> Self {
            Self
        }

        pub fn with_read_only(self, _read_only: bool) -> Self {
            self
        }

        pub fn with_max_width(self, _max_width: f32) -> Self {
            self
        }
    }

    #[derive(Clone, Debug)]
    pub enum CodeEditorViewAction {}

    pub struct CodeEditorView;

    impl CodeEditorView {
        pub fn new(
            _session_platform: Option<SessionPlatform>,
            _buffer: Option<warpui::ModelHandle<Buffer>>,
            _options: CodeEditorRenderOptions,
            _ctx: &mut ViewContext<Self>,
        ) -> Self {
            Self
        }

        pub fn with_can_show_diff_ui(self, _can_show_diff_ui: bool) -> Self {
            self
        }

        pub fn set_language_with_path<T>(&mut self, _path: T, _ctx: &mut ViewContext<Self>) {}
        pub fn starting_line_number(&self) -> Option<usize> {
            None
        }
        pub fn set_starting_line_number(&mut self, _line_number: Option<usize>) {}
        pub fn set_show_current_line_highlights(
            &mut self,
            _show: bool,
            _ctx: &mut ViewContext<Self>,
        ) {
        }
        pub fn append_at_end(&mut self, _text: &str, _ctx: &mut ViewContext<Self>) {}
        pub fn truncate(&mut self, _len: usize, _ctx: &mut ViewContext<Self>) {}
        pub fn selected_text(&self, _ctx: &AppContext) -> Option<String> {
            None
        }
        pub fn clear_selection(&mut self, _ctx: &mut ViewContext<Self>) {}
        pub fn text(&self, _ctx: &AppContext) -> String {
            String::new()
        }
        pub fn set_interaction_state<T>(&mut self, _state: T, _ctx: &mut ViewContext<Self>) {}
        pub fn reset<T>(&mut self, _state: T, _ctx: &mut ViewContext<Self>) {}
        pub fn buffer_version(&self, _ctx: &AppContext) -> usize {
            0
        }
        pub fn set_pending_scroll(&mut self, _scroll: super::scroll::ScrollTrigger) {}
    }

    impl Entity for CodeEditorView {
        type Event = CodeEditorEvent;
    }

    impl View for CodeEditorView {
        fn ui_name() -> &'static str {
            "CodeEditorView"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }

    impl TypedActionView for CodeEditorView {
        type Action = CodeEditorViewAction;

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }
}

pub mod comment_editor {
    use crate::notebooks::editor::view::RichTextEditorView;
    use warpui::{units::Pixels, View, ViewContext, ViewHandle};

    pub const DEFAULT_COMMENT_MAX_WIDTH: f32 = 480.0;
    pub type CommentEditor = ViewHandle<RichTextEditorView>;
    pub enum CommentEditorEvent {}

    pub(crate) fn create_readonly_comment_markdown_editor<V>(
        _markdown_content: &str,
        _disable_scrolling: bool,
        _max_width: Option<Pixels>,
        _ctx: &mut ViewContext<V>,
    ) -> ViewHandle<RichTextEditorView>
    where
        V: View,
    {
        unimplemented!("code editor is disabled in OSS builds")
    }
}

pub struct EditorCommentsModel;
#[derive(Clone, Debug)]
pub struct EditorReviewComment {
    pub id: crate::code_review::comments::CommentId,
    pub line: line::EditorLineLocation,
    pub diff_content: crate::code_review::comments::LineDiffContent,
    pub comment_content: String,
    pub last_update_time: chrono::DateTime<chrono::Local>,
}

impl TryFrom<crate::code_review::comments::AttachedReviewComment> for EditorReviewComment {
    type Error = ();

    fn try_from(
        comment: crate::code_review::comments::AttachedReviewComment,
    ) -> Result<Self, Self::Error> {
        let crate::code_review::comments::AttachedReviewCommentTarget::Line {
            line, content, ..
        } = comment.target
        else {
            return Err(());
        };

        Ok(Self {
            id: comment.id,
            line,
            diff_content: content,
            comment_content: comment.content,
            last_update_time: comment.last_update_time,
        })
    }
}

pub enum GutterHoverTarget {}
pub enum NavBarBehavior {
    NotClosable,
}
pub use comment_editor::{CommentEditor, CommentEditorEvent};

pub(crate) fn add_color(appearance: &Appearance) -> ColorU {
    AnsiColorIdentifier::Green
        .to_ansi_color(&appearance.theme().terminal_colors().normal)
        .into()
}

pub(crate) fn remove_color(appearance: &Appearance) -> ColorU {
    AnsiColorIdentifier::Red
        .to_ansi_color(&appearance.theme().terminal_colors().normal)
        .into()
}
