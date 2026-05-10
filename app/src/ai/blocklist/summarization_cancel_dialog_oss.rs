use warpui::{elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext};

pub fn init(_app: &mut AppContext) {}

pub enum SummarizationCancelDialogEvent {
    ConfirmCancel,
    Continue,
}

#[derive(Debug)]
pub enum SummarizationCancelDialogAction {
    ConfirmCancel,
    Continue,
}

#[derive(Default)]
pub struct SummarizationCancelDialog;

impl Entity for SummarizationCancelDialog {
    type Event = SummarizationCancelDialogEvent;
}

impl View for SummarizationCancelDialog {
    fn ui_name() -> &'static str {
        "OSSDisabledSummarizationCancelDialog"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for SummarizationCancelDialog {
    type Action = SummarizationCancelDialogAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            SummarizationCancelDialogAction::ConfirmCancel => {
                ctx.emit(SummarizationCancelDialogEvent::ConfirmCancel);
            }
            SummarizationCancelDialogAction::Continue => {
                ctx.emit(SummarizationCancelDialogEvent::Continue);
            }
        }
    }
}
