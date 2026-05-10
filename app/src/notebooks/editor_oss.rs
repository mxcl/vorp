pub mod model {
    use crate::terminal::ShellLaunchData;
    use warp_core::semantic_selection::SemanticSelection;
    use warp_editor::content::buffer::EditOrigin;
    use warp_editor::render::model::RichTextStyles;
    use warp_editor::selection::TextUnit;
    use warpui::{AppContext, Entity, ModelContext, SingletonEntity};

    #[derive(Clone)]
    pub struct FileLinkResolutionContext {
        pub working_directory: String,
        pub shell_launch_data: Option<ShellLaunchData>,
    }

    pub struct NotebooksEditorModel {
        markdown: String,
        file_link_resolution_context: Option<FileLinkResolutionContext>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum RichTextEditorModelEvent {
        ContentChanged(EditOrigin),
    }

    impl NotebooksEditorModel {
        pub fn new(
            _text_styles: RichTextStyles,
            _rte_window_id: warpui::WindowId,
            _ctx: &mut ModelContext<Self>,
        ) -> Self {
            Self {
                markdown: String::new(),
                file_link_resolution_context: None,
            }
        }

        pub fn new_unbound(_text_styles: RichTextStyles, _ctx: &mut ModelContext<Self>) -> Self {
            Self {
                markdown: String::new(),
                file_link_resolution_context: None,
            }
        }

        pub fn set_window_id(
            &mut self,
            _window_id: warpui::WindowId,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn set_file_link_resolution_context(
            &mut self,
            file_link_resolution_context: Option<FileLinkResolutionContext>,
        ) {
            self.file_link_resolution_context = file_link_resolution_context;
        }

        pub fn file_link_resolution_context(&self) -> Option<&FileLinkResolutionContext> {
            self.file_link_resolution_context.as_ref()
        }

        pub fn markdown(&self, _ctx: &AppContext) -> String {
            self.markdown.clone()
        }

        pub fn markdown_unescaped(&self, _ctx: &AppContext) -> String {
            self.markdown.clone()
        }

        pub fn reset_with_markdown(&mut self, markdown: &str, _ctx: &mut ModelContext<Self>) {
            self.markdown = markdown.to_string();
        }

        pub fn update_to_new_markdown(&mut self, markdown: &str, ctx: &mut ModelContext<Self>) {
            self.markdown = markdown.to_string();
            ctx.emit(RichTextEditorModelEvent::ContentChanged(
                EditOrigin::SystemEdit,
            ));
        }

        pub fn apply_diffs(
            &mut self,
            _diffs: Vec<ai::diff_validation::DiffDelta>,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn markdown_table_count(&self, _ctx: &AppContext) -> usize {
            0
        }

        pub fn set_interaction_state<T>(&mut self, _state: T, _ctx: &mut ModelContext<Self>) {}
    }

    impl Entity for NotebooksEditorModel {
        type Event = RichTextEditorModelEvent;
    }

    pub fn word_unit(ctx: &AppContext) -> TextUnit {
        TextUnit::Word(SemanticSelection::as_ref(ctx).word_boundary_policy())
    }
}

pub mod view {
    use crate::notebooks::{editor::model::NotebooksEditorModel, link::NotebookLinks};
    use warp_editor::render::element::VerticalExpansionBehavior;
    use warpui::elements::Empty;
    use warpui::{AppContext, Element, Entity, ModelHandle, TypedActionView, View, ViewContext};

    #[derive(Clone)]
    pub enum EditorViewEvent {
        Focused,
        TextSelectionChanged,
        ContentChanged,
        CmdEnter,
    }

    #[derive(Default)]
    pub struct RichTextEditorConfig {
        pub max_width: Option<warpui::units::Pixels>,
        pub gutter_width: Option<f32>,
        pub vertical_expansion_behavior: Option<VerticalExpansionBehavior>,
        pub embedded_objects_enabled: Option<bool>,
        pub can_execute_shell_commands: Option<bool>,
        pub disable_scrolling: bool,
        pub disable_block_insertion_menu: bool,
    }

    pub struct RichTextEditorView;

    impl RichTextEditorView {
        pub fn new(
            _parent_position_id: String,
            _model: ModelHandle<NotebooksEditorModel>,
            _links: ModelHandle<NotebookLinks>,
            _config: RichTextEditorConfig,
            _ctx: &mut ViewContext<Self>,
        ) -> Self {
            Self
        }

        pub fn selected_text(&self, _ctx: &AppContext) -> Option<String> {
            None
        }

        pub fn clear_text_selection(&mut self, _ctx: &mut ViewContext<Self>) {}
    }

    impl Entity for RichTextEditorView {
        type Event = EditorViewEvent;
    }

    impl View for RichTextEditorView {
        fn ui_name() -> &'static str {
            "RichTextEditorView"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }

    impl TypedActionView for RichTextEditorView {
        type Action = ();

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }
}

pub fn rich_text_styles(
    _appearance: &crate::appearance::Appearance,
    _font_settings: &crate::settings::FontSettings,
) -> warp_editor::render::model::RichTextStyles {
    unimplemented!("rich-text notebooks are disabled in OSS builds")
}

pub fn markdown_table_appearance(
    _appearance: &crate::appearance::Appearance,
) -> warp_editor::render::model::TableStyle {
    unimplemented!("rich-text notebooks are disabled in OSS builds")
}
