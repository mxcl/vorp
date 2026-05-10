use std::{borrow::Cow, time::Duration};

use warpui::{Entity, ModelContext};

use crate::terminal::input::message_bar::{Message, MessageProvider};

use super::agent_message_bar::AgentMessageArgs;

pub struct EphemeralMessage {
    id: Option<Cow<'static, str>>,
}

#[derive(Clone, Copy)]
pub enum DismissalStrategy {
    UntilExplicitlyDismissed,
    Timer(Duration),
}

impl EphemeralMessage {
    pub fn new(_message: Message, _dismissal: DismissalStrategy) -> Self {
        Self { id: None }
    }

    pub fn with_id(mut self, id: impl Into<Cow<'static, str>>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_duration(self, _duration: Duration) -> Self {
        self
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_ref().map(|id| id.as_ref())
    }
}

pub struct EphemeralMessageModel;

#[derive(Debug, Clone, Copy)]
pub enum EphemeralMessageModelEvent {
    MessageChanged,
}

impl EphemeralMessageModel {
    pub fn new() -> Self {
        Self
    }

    pub fn current_message(&self) -> Option<&EphemeralMessage> {
        None
    }

    pub fn show_ephemeral_message(
        &mut self,
        _message: EphemeralMessage,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn show_info_ephemeral_message(
        &mut self,
        _message: impl Into<Cow<'static, str>>,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn try_dismiss_explicit_message(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn clear_message(&mut self, _ctx: &mut ModelContext<Self>) {}
}

impl Entity for EphemeralMessageModel {
    type Event = EphemeralMessageModelEvent;
}

impl MessageProvider<AgentMessageArgs<'_>> for EphemeralMessageModel {
    fn produce_message(&self, _args: AgentMessageArgs<'_>) -> Option<Message> {
        None
    }
}
