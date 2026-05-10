use crate::{cloud_object::ObjectType, code_review::diff_state::DiffMode};
use warpui::{elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext};

pub mod mixer {
    use super::{DiffMode, ObjectType};

    #[derive(Clone, PartialEq)]
    pub enum AIContextMenuSearchableAction {
        InsertFilePath {
            file_path: String,
        },
        InsertText {
            text: String,
        },
        InsertDriveObject {
            object_type: ObjectType,
            object_uid: String,
        },
        InsertPlan {
            ai_document_uid: String,
        },
        InsertDiffSet {
            diff_mode: DiffMode,
        },
        InsertConversation {
            conversation_id: String,
        },
        InsertSkill {
            name: String,
        },
    }

    impl std::fmt::Debug for AIContextMenuSearchableAction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("ContextMenuSearchableAction")
        }
    }

    pub struct AIContextMenuMixer;
}

pub mod search {
    const MAX_NEW_SPACES: usize = 2;

    pub fn is_valid_search_query(is_navigation: bool, prev_query: &str, query: &str) -> bool {
        if query.contains('\n') || query.contains("  ") {
            return false;
        }

        if is_navigation {
            let new_chars = query.chars().skip(prev_query.len());
            return new_chars.filter(|c| *c == ' ').count() < MAX_NEW_SPACES;
        }

        true
    }
}

pub mod view {
    use super::mixer::AIContextMenuSearchableAction;
    use super::*;

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum AIContextMenuPosition {
        AtButton,
        AtCursor,
    }

    impl std::fmt::Debug for AIContextMenuPosition {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("ContextMenuPosition")
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum AIContextMenuCategory {
        CurrentFolderFiles,
        RepoFiles,
        Commands,
        Blocks,
        Workflows,
        Notebooks,
        Plans,
        Diffs,
        Docs,
        Tasks,
        Rules,
        Servers,
        Terminal,
        Web,
        RecentDiff,
        RecentBlock,
        Code,
        DiffSet,
        Conversations,
        Skills,
    }

    impl std::fmt::Debug for AIContextMenuCategory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("ContextMenuCategory")
        }
    }

    impl AIContextMenuCategory {
        pub fn name(&self) -> &'static str {
            ""
        }

        pub fn icon(&self) -> &'static str {
            ""
        }
    }

    #[derive(Clone)]
    pub enum AIContextMenuAction {
        Prev,
        Next,
        SelectCurrentItem,
        ResultAccepted {
            action: AIContextMenuSearchableAction,
        },
        CategorySelected {
            category: AIContextMenuCategory,
        },
        Close,
    }

    impl std::fmt::Debug for AIContextMenuAction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("ContextMenuAction")
        }
    }

    pub enum AIContextMenuEvent {
        Close {
            query_length: usize,
            item_count: Option<usize>,
        },
        ResultAccepted {
            action: AIContextMenuSearchableAction,
            query_length: usize,
            item_count: Option<usize>,
        },
        CategorySelected {
            category: AIContextMenuCategory,
        },
    }

    pub struct AIContextMenu {
        query: String,
        is_ai_or_autodetect_mode: bool,
        is_shared_session_viewer: bool,
        is_in_ambient_agent: bool,
        is_cli_agent_input: bool,
    }

    impl AIContextMenu {
        pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
            Self {
                query: String::new(),
                is_ai_or_autodetect_mode: true,
                is_shared_session_viewer: false,
                is_in_ambient_agent: false,
                is_cli_agent_input: false,
            }
        }

        pub(crate) fn get_categories_for_mode(
            _is_ai_or_autodetect_mode: bool,
            _is_shared_session_viewer: bool,
            _is_in_ambient_agent: bool,
            _is_cli_agent_input: bool,
            _app: &AppContext,
        ) -> Vec<AIContextMenuCategory> {
            Vec::new()
        }

        pub fn set_is_shared_session_viewer(
            &mut self,
            is_viewer: bool,
            _ctx: &mut ViewContext<Self>,
        ) {
            self.is_shared_session_viewer = is_viewer;
        }

        pub fn set_is_in_ambient_agent(&mut self, is_ambient: bool, _ctx: &mut ViewContext<Self>) {
            self.is_in_ambient_agent = is_ambient;
        }

        pub fn set_is_cli_agent_input(
            &mut self,
            is_cli_agent_input: bool,
            _ctx: &mut ViewContext<Self>,
        ) {
            self.is_cli_agent_input = is_cli_agent_input;
        }

        pub fn set_input_mode(
            &mut self,
            is_ai_or_autodetect_mode: bool,
            _ctx: &mut ViewContext<Self>,
        ) {
            self.is_ai_or_autodetect_mode = is_ai_or_autodetect_mode;
        }

        pub fn select_current_item(&mut self, _ctx: &mut ViewContext<Self>) {}

        pub fn close(&mut self, ctx: &mut ViewContext<Self>) {
            ctx.emit(AIContextMenuEvent::Close {
                query_length: self.query.len(),
                item_count: Some(0),
            });
            ctx.notify();
        }

        pub fn reset_menu_state(&mut self, ctx: &mut ViewContext<Self>) {
            self.query.clear();
            ctx.notify();
        }

        pub fn update_search_query(&mut self, query: String, ctx: &mut ViewContext<Self>) {
            self.query = query;
            ctx.notify();
        }

        pub fn should_render(&self, _app: &AppContext) -> bool {
            false
        }
    }

    impl Entity for AIContextMenu {
        type Event = AIContextMenuEvent;
    }

    impl TypedActionView for AIContextMenu {
        type Action = AIContextMenuAction;

        fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
            match action {
                AIContextMenuAction::ResultAccepted { action } => {
                    ctx.emit(AIContextMenuEvent::ResultAccepted {
                        action: action.clone(),
                        query_length: self.query.len(),
                        item_count: Some(0),
                    });
                }
                AIContextMenuAction::CategorySelected { category } => {
                    ctx.emit(AIContextMenuEvent::CategorySelected {
                        category: *category,
                    });
                }
                AIContextMenuAction::Close => self.close(ctx),
                AIContextMenuAction::Prev
                | AIContextMenuAction::Next
                | AIContextMenuAction::SelectCurrentItem => {}
            }
        }
    }

    impl View for AIContextMenu {
        fn ui_name() -> &'static str {
            "ContextMenuView"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }
}

pub fn safe_truncate(s: &mut String, new_len: usize) {
    if new_len >= s.len() {
        return;
    }

    let safe_len = floor_char_boundary(s, new_len);
    s.truncate(safe_len);
}

pub fn floor_char_boundary(original_string: &str, idx: usize) -> usize {
    if idx >= original_string.len() {
        original_string.len()
    } else {
        let mut curr = idx;
        while curr > 0 && !original_string.is_char_boundary(curr) {
            curr -= 1;
        }
        curr
    }
}
