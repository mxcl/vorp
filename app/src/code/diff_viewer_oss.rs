#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    FullPane,
    Embedded {
        max_height: i32,
    },
    InlineBanner {
        max_height: i32,
        is_expanded: bool,
        is_dismissed: bool,
    },
}

impl DisplayMode {
    pub fn with_embedded(max_height: f32) -> Self {
        Self::Embedded {
            max_height: max_height as i32,
        }
    }

    pub fn with_inline_banner(max_height: f32) -> Self {
        Self::InlineBanner {
            max_height: max_height as i32,
            is_expanded: false,
            is_dismissed: false,
        }
    }

    pub fn max_height(&self) -> Option<f32> {
        match self {
            Self::FullPane => None,
            Self::Embedded { max_height } | Self::InlineBanner { max_height, .. } => {
                Some(*max_height as f32)
            }
        }
    }

    pub fn title(&self) -> Option<&str> {
        match self {
            Self::InlineBanner { .. } => Some("Suggested fixes based on your last command:"),
            _ => None,
        }
    }

    pub fn is_full_pane(&self) -> bool {
        matches!(self, Self::FullPane)
    }

    pub fn is_embedded(&self) -> bool {
        matches!(self, Self::Embedded { .. })
    }

    pub fn is_inline_banner(&self) -> bool {
        matches!(self, Self::InlineBanner { .. })
    }
}
