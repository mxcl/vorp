use std::path::PathBuf;

use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, TypedActionView, View, ViewContext};

pub struct NewWorktreeModal;

pub fn init(_app: &mut AppContext) {}

pub enum NewWorktreeModalEvent {
    Close,
    Submit {
        repo: String,
        branch: String,
        worktree_branch_name: Option<String>,
    },
    PickNewRepo,
}

#[derive(Clone, Copy, Debug)]
pub enum NewWorktreeModalAction {
    Cancel,
    Open,
    ToggleAutogenerate,
    Escape,
}

impl NewWorktreeModal {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn on_open(&mut self, _cwd: Option<PathBuf>, _ctx: &mut ViewContext<Self>) {}

    pub fn on_close(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn on_new_repo_selected(&mut self, _path: PathBuf, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for NewWorktreeModal {
    type Event = NewWorktreeModalEvent;
}

impl View for NewWorktreeModal {
    fn ui_name() -> &'static str {
        "NewWorktreeModal"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for NewWorktreeModal {
    type Action = NewWorktreeModalAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        if matches!(
            action,
            NewWorktreeModalAction::Cancel | NewWorktreeModalAction::Escape
        ) {
            ctx.emit(NewWorktreeModalEvent::Close);
        }
    }
}
