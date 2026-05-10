use markdown_parser::FormattedTextLine;
use warpui::assets::asset_cache::AssetSource;

use super::{CTAButton, Slide};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OzLaunchSlide {
    Empty,
}

impl Slide for OzLaunchSlide {
    fn modal_title(&self) -> String {
        String::new()
    }

    fn modal_subtext_paragraphs(&self) -> Vec<FormattedTextLine> {
        Vec::new()
    }

    fn first() -> Self {
        Self::Empty
    }

    fn next(&self) -> Option<Self> {
        None
    }

    fn prev(&self) -> Option<Self> {
        None
    }

    fn display_text(&self) -> Option<&'static str> {
        None
    }

    fn short_label(&self) -> &'static str {
        ""
    }

    fn title(&self) -> &'static str {
        ""
    }

    fn title_icon(&self) -> Option<crate::ui_components::icons::Icon> {
        None
    }

    fn content(&self) -> &'static str {
        ""
    }

    fn image(&self) -> AssetSource {
        AssetSource::Bundled { path: "" }
    }

    fn all() -> Vec<Self> {
        vec![Self::Empty]
    }

    fn cta_button(&self) -> CTAButton<Self> {
        CTAButton::close("")
    }
}

pub fn init(_app: &mut warpui::AppContext) {}
