//! OSS no-op conversation list panel.

pub mod view {
    use crate::ai::agent::conversation::AIConversationId;
    use warpui::elements::Empty;
    use warpui::{AppContext, Element, Entity, TypedActionView, View, ViewContext};

    #[derive(Debug, Clone)]
    pub enum Event {
        NewConversationInNewTab,
        ShowDeleteConfirmationDialog {
            conversation_id: AIConversationId,
            conversation_title: String,
            terminal_view_id: Option<warpui::EntityId>,
        },
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum ConversationListViewAction {}

    pub struct ConversationListView;

    pub fn register_conversation_list_view_bindings(_app: &mut AppContext) {}

    impl ConversationListView {
        pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
            Self
        }

        pub fn on_left_panel_focused(&mut self, _ctx: &mut ViewContext<Self>) {}
    }

    impl View for ConversationListView {
        fn ui_name() -> &'static str {
            "ConversationListView"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }

    impl Entity for ConversationListView {
        type Event = Event;
    }

    impl TypedActionView for ConversationListView {
        type Action = ConversationListViewAction;

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }
}
