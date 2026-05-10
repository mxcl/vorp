use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct LineCount(usize);

impl LineCount {
    pub fn as_usize(self) -> usize {
        self.0
    }

    pub fn as_u32(self) -> u32 {
        self.0 as u32
    }

    pub fn saturating_sub(self, rhs: &Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }
}

impl From<usize> for LineCount {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Add for LineCount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for LineCount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

pub mod editor {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NavigationKey {
        Up,
        Down,
        Left,
        Right,
        PageUp,
        PageDown,
        Tab,
        ShiftTab,
    }
}

pub mod content {
    pub mod buffer {
        use super::super::content::markdown::MarkdownStyle;
        use string_offset::CharOffset;
        use warpui::text::word_boundaries::WordBoundariesPolicy;
        use warpui::{Entity, ModelContext};

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum EditOrigin {
            UserTyped,
            UserInitiated,
            SystemEdit,
            SyncedTerminalInput,
            RemoteEdit,
        }

        #[derive(Clone, Debug, Default)]
        pub struct Buffer;

        #[derive(Clone, Debug, Default)]
        pub struct BufferSnapshot;

        #[derive(Clone, Debug, Default)]
        pub struct InitialBufferState {
            pub text: String,
        }

        impl InitialBufferState {
            pub fn plain_text(text: &str) -> Self {
                Self {
                    text: text.to_string(),
                }
            }
        }

        impl Buffer {
            pub fn export_to_markdown<T>(
                _parsed: T,
                _embedded_item_conversion: Option<fn()>,
                _style: MarkdownStyle<'_>,
            ) -> String {
                String::new()
            }

            pub fn chars_at(&self, _offset: CharOffset) -> Result<std::str::Chars<'static>, ()> {
                Ok("".chars())
            }

            pub fn word_starts_backward_from_offset_inclusive(
                &self,
                _point: CharOffset,
            ) -> Result<WordBoundaryIter, ()> {
                Ok(WordBoundaryIter::default())
            }

            pub fn word_ends_from_offset_exclusive(
                &self,
                _point: CharOffset,
            ) -> Result<WordBoundaryIter, ()> {
                Ok(WordBoundaryIter::default())
            }
        }

        impl Entity for Buffer {
            type Event = ();
        }

        #[derive(Default)]
        pub struct WordBoundaryIter(Option<CharOffset>);

        impl WordBoundaryIter {
            pub fn with_policy(self, _policy: &WordBoundariesPolicy) -> Self {
                self
            }
        }

        impl Iterator for WordBoundaryIter {
            type Item = CharOffset;

            fn next(&mut self) -> Option<Self::Item> {
                self.0.take()
            }
        }

        pub trait ToBufferCharOffset {
            fn to_buffer_char_offset(self, buffer: &Buffer) -> CharOffset;
        }

        pub trait ToBufferPoint {
            fn to_buffer_point(self, buffer: &Buffer) -> CharOffset;
        }

        impl ToBufferCharOffset for CharOffset {
            fn to_buffer_char_offset(self, _buffer: &Buffer) -> CharOffset {
                self
            }
        }

        impl ToBufferPoint for CharOffset {
            fn to_buffer_point(self, _buffer: &Buffer) -> CharOffset {
                self
            }
        }

        #[allow(dead_code)]
        fn _assert_model_context(_: &mut ModelContext<Buffer>) {}
    }

    pub mod edit {
        use std::path::Path;
        use warpui::assets::asset_cache::AssetSource;

        #[derive(Clone, Debug, Default)]
        pub struct PreciseDelta;

        pub fn resolve_asset_source_relative_to_directory(
            source: &str,
            current_working_directory: Option<&Path>,
        ) -> AssetSource {
            let path = current_working_directory
                .map(|cwd| cwd.join(source).to_string_lossy().to_string())
                .unwrap_or_else(|| source.to_string());
            AssetSource::LocalFile { path }
        }
    }

    pub mod markdown {
        use warpui::AppContext;

        #[derive(Clone, Copy)]
        pub enum MarkdownStyle<'a> {
            Internal,
            Export {
                app_context: Option<&'a AppContext>,
                should_not_escape_markdown_punctuation: bool,
            },
        }
    }

    pub mod mermaid_diagram {
        use warpui::assets::asset_cache::AssetSource;

        pub fn mermaid_asset_source(source: &str) -> AssetSource {
            AssetSource::Raw {
                id: source.to_string(),
            }
        }
    }

    pub mod text {
        pub use crate::warp_editor_shim::LineCount;

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum IndentUnit {
            Space(usize),
            Tab,
        }

        #[derive(Clone, Debug)]
        pub enum IndentBehavior {
            Ignore,
            TabIndent(IndentUnit),
        }

        impl super::buffer::EditOrigin {
            pub fn from_user(&self) -> bool {
                matches!(self, Self::UserTyped | Self::UserInitiated)
            }
        }

        pub fn format_image_markdown(alt_text: &str, source: &str, title: Option<&str>) -> String {
            match title.filter(|title| !title.is_empty()) {
                Some(title) => {
                    let escaped = title.replace('\\', "\\\\").replace('"', "\\\"");
                    format!("![{alt_text}]({source} \"{escaped}\")")
                }
                None => format!("![{alt_text}]({source})"),
            }
        }
    }

    pub mod version {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
        pub struct BufferVersion(pub u64);
    }
}

pub mod decoration {
    use crate::warp_editor_shim::content::{
        buffer::BufferSnapshot, edit::PreciseDelta, version::BufferVersion,
    };
    use warpui::ModelContext;

    pub trait DecorationLayer {
        fn update_internal_state_with_delta(
            &mut self,
            deltas: &[PreciseDelta],
            content_version: BufferVersion,
            content: BufferSnapshot,
            ctx: &mut ModelContext<Self>,
        ) where
            Self: Sized;
    }
}

pub mod model {
    #[derive(Clone, Debug, Default)]
    pub struct RichTextEditorModel;
}

pub mod render {
    pub mod element {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum VerticalExpansionBehavior {
            InfiniteHeight,
            GrowToMaxHeight,
        }
    }

    pub mod model {
        use warpui::color::ColorU;
        use warpui::units::Pixels;

        pub use crate::warp_editor_shim::LineCount;

        #[derive(Clone, Copy, Debug)]
        pub struct InlineCodeStyle {
            pub background: ColorU,
            pub font_color: ColorU,
        }

        #[derive(Clone, Debug)]
        pub struct RichTextStyles {
            pub inline_code_style: InlineCodeStyle,
        }

        impl RichTextStyles {
            pub fn base_line_height(&self) -> Pixels {
                Pixels::new(20.0)
            }
        }

        impl Default for RichTextStyles {
            fn default() -> Self {
                Self {
                    inline_code_style: InlineCodeStyle {
                        background: ColorU::new(0, 0, 0, 0),
                        font_color: ColorU::new(0, 0, 0, 255),
                    },
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct TableStyle {
            pub border_color: ColorU,
            pub header_background: ColorU,
            pub cell_background: ColorU,
            pub alternate_row_background: Option<ColorU>,
            pub text_color: ColorU,
            pub header_text_color: ColorU,
            pub cell_padding: f32,
            pub outer_border: bool,
            pub column_dividers: bool,
            pub row_dividers: bool,
        }

        impl Default for TableStyle {
            fn default() -> Self {
                Self {
                    border_color: ColorU::new(0, 0, 0, 255),
                    header_background: ColorU::new(0, 0, 0, 0),
                    cell_background: ColorU::new(0, 0, 0, 0),
                    alternate_row_background: None,
                    text_color: ColorU::new(0, 0, 0, 255),
                    header_text_color: ColorU::new(0, 0, 0, 255),
                    cell_padding: 0.0,
                    outer_border: false,
                    column_dividers: false,
                    row_dividers: false,
                }
            }
        }
    }
}

pub mod selection {
    #[derive(Clone, Debug)]
    pub enum TextUnit<T = warpui::text::word_boundaries::WordBoundariesPolicy> {
        Word(T),
    }
}
