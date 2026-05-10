use warpui::elements::Empty;
use warpui::{AppContext, Element, Entity, TypedActionView, View, ViewContext};

pub fn init(_app: &mut AppContext) {}

pub enum LocalCodeEditorEvent {}

#[derive(Clone, Debug)]
pub enum LocalCodeEditorAction {}

pub struct LocalCodeEditorView;

impl LocalCodeEditorView {
    pub fn has_unsaved_changes(&self, _ctx: &AppContext) -> bool {
        false
    }
}

impl Entity for LocalCodeEditorView {
    type Event = LocalCodeEditorEvent;
}

impl View for LocalCodeEditorView {
    fn ui_name() -> &'static str {
        "LocalCodeEditorView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for LocalCodeEditorView {
    type Action = LocalCodeEditorAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}
