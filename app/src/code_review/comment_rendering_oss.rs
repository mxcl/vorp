use std::{path::Path, rc::Rc};

use crate::code_review::comments::AttachedReviewComment;
use warpui::{
    elements::{Empty, MouseStateHandle},
    AppContext, Element, EventContext, View, ViewContext,
};

pub(crate) struct HeaderClickHandler {
    pub mouse_state: MouseStateHandle,
    pub on_click: Rc<dyn Fn(&mut EventContext) + 'static>,
}

pub(crate) struct CommentViewCard {
    source: AttachedReviewComment,
    is_collapsed: bool,
}

impl CommentViewCard {
    pub(crate) fn new<V: View>(
        source: AttachedReviewComment,
        _always_use_static_diff: bool,
        _disable_scrolling: bool,
        _max_width: Option<warpui::units::Pixels>,
        _repo_path: Option<&Path>,
        _ctx: &mut ViewContext<V>,
    ) -> Self {
        Self {
            source,
            is_collapsed: false,
        }
    }

    pub(crate) fn toggle_collapsed(&mut self) {
        self.is_collapsed = !self.is_collapsed;
    }

    pub(crate) fn is_collapsed(&self) -> bool {
        self.is_collapsed
    }

    pub(crate) fn render(
        &self,
        _editor_lens_element: Option<Box<dyn Element>>,
        _header_trailing_element: Option<Box<dyn Element>>,
        _metadata_trailing_element: Option<Box<dyn Element>>,
        _on_header_click: Option<&HeaderClickHandler>,
        _app: &AppContext,
    ) -> Box<dyn Element> {
        Empty::new().finish()
    }

    pub(crate) fn source(&self) -> &AttachedReviewComment {
        &self.source
    }
}
