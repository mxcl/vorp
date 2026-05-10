use std::{cell::Ref, sync::Arc};

use languages::Language;
use rangemap::{RangeMap, RangeSet};
use string_offset::CharOffset;
use warp_editor::{
    content::{
        buffer::{Buffer, BufferSnapshot},
        edit::PreciseDelta,
        text::IndentUnit,
        version::BufferVersion,
    },
    decoration::DecorationLayer,
};
use warpui::{
    color::ColorU, text::point::Point, AppContext, Entity, ModelContext, WeakModelHandle,
};

#[derive(Clone, Copy)]
pub struct ColorMap {
    pub keyword_color: ColorU,
    pub function_color: ColorU,
    pub string_color: ColorU,
    pub type_color: ColorU,
    pub number_color: ColorU,
    pub comment_color: ColorU,
    pub property_color: ColorU,
    pub tag_color: ColorU,
}

pub enum DecorationStateEvent {
    DecorationUpdated { version: BufferVersion },
}

#[derive(Debug)]
pub struct IndentDelta {
    pub delta: u8,
}

pub struct SyntaxTreeState {
    indent_unit: Option<IndentUnit>,
    bracket_pairs: Vec<(char, char)>,
    comment_prefix: Option<String>,
}

impl SyntaxTreeState {
    pub fn new(
        _buffer_handle: WeakModelHandle<Buffer>,
        _buffer_version: BufferVersion,
        _color_map: ColorMap,
    ) -> Self {
        Self {
            indent_unit: None,
            bracket_pairs: Vec::new(),
            comment_prefix: None,
        }
    }

    pub fn set_language(&mut self, language: Arc<Language>) {
        self.indent_unit = Some(language.indent_unit);
        self.bracket_pairs = language.bracket_pairs.clone();
        self.comment_prefix = language.comment_prefix.clone();
    }

    pub fn has_supported_highlighting(&self) -> bool {
        false
    }

    pub fn indent_unit(&self) -> Option<IndentUnit> {
        self.indent_unit
    }

    pub fn bracket_pairs(&self) -> Option<&[(char, char)]> {
        Some(self.bracket_pairs.as_slice())
    }

    pub fn comment_prefix(&self) -> Option<&str> {
        self.comment_prefix.as_deref()
    }

    pub fn highlights_in_ranges(
        &self,
        _ranges: RangeSet<CharOffset>,
        _render_content_version: Option<BufferVersion>,
        _ctx: &AppContext,
    ) -> Option<Ref<'_, RangeMap<CharOffset, ColorU>>> {
        None
    }

    pub fn indentation_at_point(&self, _point: Point, _ctx: &AppContext) -> Option<IndentDelta> {
        None
    }

    pub fn set_color_map(&mut self, _color_map: ColorMap) {}
}

impl DecorationLayer for SyntaxTreeState {
    fn update_internal_state_with_delta(
        &mut self,
        _deltas: &[PreciseDelta],
        _content_version: BufferVersion,
        _content: BufferSnapshot,
        _ctx: &mut ModelContext<Self>,
    ) {
    }
}

impl Entity for SyntaxTreeState {
    type Event = DecorationStateEvent;
}
