pub(crate) mod toast_stack {
    use warpui::elements::{Element, Empty};
    use warpui::{AppContext, Entity, TypedActionView, View, ViewContext};

    use super::NotificationId;

    pub struct AgentNotificationToastStack;

    impl Entity for AgentNotificationToastStack {
        type Event = ();
    }

    #[derive(Debug)]
    pub enum AgentNotificationToastAction {
        CancelDismissalTimeout(NotificationId),
        StartDismissalTimeout(NotificationId),
        Click(NotificationId),
        Dismiss(NotificationId),
        ToggleMessageExpanded(NotificationId),
    }

    impl AgentNotificationToastStack {
        pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
            Self
        }

        pub fn set_mailbox_open(&mut self, _open: bool, _ctx: &mut ViewContext<Self>) {}
    }

    impl TypedActionView for AgentNotificationToastStack {
        type Action = AgentNotificationToastAction;

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }

    impl View for AgentNotificationToastStack {
        fn ui_name() -> &'static str {
            "AgentNotificationToastStack"
        }

        fn render(&self, _ctx: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }
}

pub(crate) mod view {
    use warpui::elements::{Element, Empty};
    use warpui::{AppContext, Entity, EntityId, TypedActionView, View, ViewContext};

    use super::NotificationFilter;

    pub struct NotificationMailboxView;

    impl Entity for NotificationMailboxView {
        type Event = NotificationMailboxViewEvent;
    }

    #[derive(Debug, Clone)]
    pub enum NotificationMailboxViewEvent {
        NavigateToTerminal { terminal_view_id: EntityId },
        Dismissed,
    }

    #[derive(Debug)]
    pub enum NotificationMailboxViewAction {
        SetFilter(NotificationFilter),
        MarkAllRead,
        Dismiss,
        SelectPrevious,
        SelectNext,
        CycleFilter,
        ActivateSelected,
    }

    impl NotificationMailboxView {
        pub fn init(_app: &mut AppContext) {}

        pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
            Self
        }

        pub fn reset_for_open(&mut self, _select_first: bool, _ctx: &mut ViewContext<Self>) {}
    }

    impl TypedActionView for NotificationMailboxView {
        type Action = NotificationMailboxViewAction;

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }

    impl View for NotificationMailboxView {
        fn ui_name() -> &'static str {
            "NotificationMailboxView"
        }

        fn render(&self, _ctx: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }
}

pub(crate) mod item {
    pub(crate) use super::{
        NotificationCategory, NotificationFilter, NotificationId, NotificationItem,
        NotificationItems, NotificationOrigin, NotificationSourceAgent,
    };
}

use instant::Instant;
use warpui::EntityId;

use crate::ai::agent::conversation::AIConversationId;
use crate::ai::artifacts::Artifact;
use crate::terminal::CLIAgent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NotificationId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationCategory {
    Complete,
    Request,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationFilter {
    All,
    Unread,
    Errors,
}

impl NotificationFilter {
    pub(crate) fn label(&self) -> &'static str {
        match self {
            NotificationFilter::All => "All tabs",
            NotificationFilter::Unread => "Unread",
            NotificationFilter::Errors => "Errors",
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum NotificationSourceAgent {
    Oz { is_ambient: bool },
    CLI { agent: CLIAgent, is_ambient: bool },
}

impl NotificationSourceAgent {
    pub fn is_ambient(&self) -> bool {
        match self {
            NotificationSourceAgent::Oz { is_ambient }
            | NotificationSourceAgent::CLI { is_ambient, .. } => *is_ambient,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotificationOrigin {
    Conversation(AIConversationId),
    CLISession(EntityId),
}

#[derive(Debug, Clone)]
pub struct NotificationItem {
    pub id: NotificationId,
    pub origin: NotificationOrigin,
    pub title: String,
    pub message: String,
    pub category: NotificationCategory,
    pub agent: NotificationSourceAgent,
    pub is_read: bool,
    pub created_at: Instant,
    pub terminal_view_id: EntityId,
    pub artifacts: Vec<Artifact>,
    pub branch: Option<String>,
}

impl NotificationItem {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        title: String,
        message: String,
        category: NotificationCategory,
        agent: NotificationSourceAgent,
        origin: NotificationOrigin,
        is_read: bool,
        terminal_view_id: EntityId,
        artifacts: Vec<Artifact>,
        branch: Option<String>,
    ) -> Self {
        Self {
            id: NotificationId,
            origin,
            title,
            message,
            category,
            agent,
            is_read,
            created_at: Instant::now(),
            terminal_view_id,
            artifacts,
            branch,
        }
    }
}

#[derive(Debug, Default)]
pub struct NotificationItems {
    items: Vec<NotificationItem>,
}

impl NotificationItems {
    pub(crate) fn push(&mut self, item: NotificationItem) {
        self.items.insert(0, item);
    }

    pub(crate) fn remove_by_origin(&mut self, key: NotificationOrigin) -> bool {
        let before = self.items.len();
        self.items.retain(|item| item.origin != key);
        self.items.len() != before
    }

    pub(crate) fn items_filtered(
        &self,
        filter: NotificationFilter,
    ) -> impl Iterator<Item = &NotificationItem> {
        self.items.iter().filter(move |item| match filter {
            NotificationFilter::All => true,
            NotificationFilter::Unread => !item.is_read,
            NotificationFilter::Errors => item.category == NotificationCategory::Error,
        })
    }

    pub(crate) fn filtered_count(&self, filter: NotificationFilter) -> usize {
        self.items_filtered(filter).count()
    }

    pub(crate) fn visible_filters(&self) -> Vec<NotificationFilter> {
        [
            NotificationFilter::All,
            NotificationFilter::Unread,
            NotificationFilter::Errors,
        ]
        .into_iter()
        .filter(|filter| *filter == NotificationFilter::All || self.filtered_count(*filter) > 0)
        .collect()
    }

    pub(crate) fn get_by_id(&self, id: NotificationId) -> Option<&NotificationItem> {
        self.items.iter().find(|item| item.id == id)
    }

    pub(crate) fn mark_all_terminal_view_items_as_read(
        &mut self,
        terminal_view_id: EntityId,
    ) -> bool {
        let mut any_changed = false;
        for item in &mut self.items {
            if item.terminal_view_id == terminal_view_id && !item.is_read {
                item.is_read = true;
                any_changed = true;
            }
        }
        any_changed
    }

    pub(crate) fn mark_item_read(&mut self, id: NotificationId) -> bool {
        if let Some(item) = self
            .items
            .iter_mut()
            .find(|item| item.id == id && !item.is_read)
        {
            item.is_read = true;
            true
        } else {
            false
        }
    }

    pub(crate) fn mark_all_items_read(&mut self) -> bool {
        let mut any_changed = false;
        for item in &mut self.items {
            if !item.is_read {
                item.is_read = true;
                any_changed = true;
            }
        }
        any_changed
    }

    pub(crate) fn has_unread_for_terminal_view(&self, terminal_view_id: EntityId) -> bool {
        self.items
            .iter()
            .any(|item| item.terminal_view_id == terminal_view_id && !item.is_read)
    }
}
