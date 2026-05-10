pub mod content;
pub mod decoration;
pub mod editor;
#[cfg(feature = "mermaid_rendering")]
extern crate mermaid_to_svg as mermaid_to_svg_crate;
#[cfg(feature = "mermaid_rendering")]
mod mermaid_to_svg {
    pub use crate::mermaid_to_svg_crate::*;
}
#[cfg(not(feature = "mermaid_rendering"))]
mod mermaid_to_svg {
    pub struct MermaidTheme;

    impl MermaidTheme {
        pub fn light() -> Self {
            Self
        }
    }

    pub fn is_mermaid_diagram(_language: &str) -> bool {
        false
    }

    pub fn render_mermaid_to_svg(
        _source: &str,
        _theme: Option<&MermaidTheme>,
    ) -> Result<String, anyhow::Error> {
        anyhow::bail!("Mermaid rendering is not available in this build")
    }
}
pub mod model;
pub mod multiline;
mod parallel_util;
pub mod render;
pub mod search;
pub mod selection;
