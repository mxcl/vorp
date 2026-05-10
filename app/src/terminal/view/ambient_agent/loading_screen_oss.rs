use std::rc::Rc;

use parking_lot::RwLock;
use warp_core::ui::appearance::Appearance;
use warpui::elements::shimmering_text::ShimmeringTextStateHandle;
use warpui::elements::{Element, Empty, MouseStateHandle, SelectionHandle};
use warpui::{AppContext, ModelHandle};

use crate::ai::agent_tips::AITipModel;
use crate::terminal::view::ambient_agent::CloudModeTip;

pub fn render_cloud_mode_loading_screen(
    _message: &str,
    _appearance: &Appearance,
    _shimmer_handle: &ShimmeringTextStateHandle,
    _tip_model: &ModelHandle<AITipModel<CloudModeTip>>,
    _app: &AppContext,
) -> Box<dyn Element> {
    Empty::new().finish()
}

pub fn render_cloud_mode_error_screen(
    _error_message: &str,
    _appearance: &Appearance,
    _selection_handle: &SelectionHandle,
    _selected_text: &Rc<RwLock<Option<String>>>,
    _app: &AppContext,
) -> Box<dyn Element> {
    Empty::new().finish()
}

pub fn render_cloud_mode_github_auth_required_screen(
    _auth_url: &str,
    _appearance: &Appearance,
    _auth_button_mouse_state: &MouseStateHandle,
    _app: &AppContext,
) -> Box<dyn Element> {
    Empty::new().finish()
}

pub fn render_cloud_mode_cancelled_screen(_appearance: &Appearance) -> Box<dyn Element> {
    Empty::new().finish()
}
