use warp_core::ui::appearance::Appearance;
use warpui::{prelude::Empty, Element};

pub fn render_loading_footer(_appearance: &Appearance) -> Box<dyn Element> {
    Empty::new().finish()
}

pub fn render_error_footer(_error_message: &str, _appearance: &Appearance) -> Box<dyn Element> {
    Empty::new().finish()
}
