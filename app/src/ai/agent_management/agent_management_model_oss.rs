use warpui::{Entity, EntityId, ModelContext, SingletonEntity, WindowId};

use crate::ai::agent::conversation::{AIConversationId, ConversationStatus};
use crate::ai::agent_management::notifications::{NotificationId, NotificationItems};
use crate::ai::artifacts::Artifact;

pub struct AgentNotificationsModel {
    notifications: NotificationItems,
}

impl Entity for AgentNotificationsModel {
    type Event = AgentManagementEvent;
}

impl SingletonEntity for AgentNotificationsModel {}

impl AgentNotificationsModel {
    pub(crate) fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self {
            notifications: NotificationItems::default(),
        }
    }

    pub(crate) fn notifications(&self) -> &NotificationItems {
        &self.notifications
    }

    pub(crate) fn mark_item_read(&mut self, _id: NotificationId, _ctx: &mut ModelContext<Self>) {}

    pub(crate) fn mark_all_items_read(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub(crate) fn mark_items_from_terminal_view_read(
        &mut self,
        _terminal_view_id: EntityId,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub(crate) fn flush_pending_artifacts(
        &mut self,
        _conversation_id: AIConversationId,
    ) -> Vec<Artifact> {
        Vec::new()
    }
}

#[derive(Clone, Debug)]
pub enum AgentManagementEvent {
    ConversationNeedsAttention {
        window_id: WindowId,
        tab_index: usize,
        terminal_view_id: EntityId,
        conversation_id: AIConversationId,
    },
    NotificationAdded {
        id: NotificationId,
    },
    NotificationUpdated,
    AllNotificationsMarkedRead,
}

impl ConversationStatus {
    pub fn should_trigger_notification(&self) -> bool {
        matches!(
            self,
            ConversationStatus::Success
                | ConversationStatus::Blocked { .. }
                | ConversationStatus::Error
        )
    }
}
