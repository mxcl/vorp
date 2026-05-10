use std::{path::Path, sync::Arc};

use warp_editor::content::text::IndentUnit;

pub struct Language {
    pub indent_unit: IndentUnit,
    pub comment_prefix: Option<String>,
    pub bracket_pairs: Vec<(char, char)>,
    display_name: String,
}

impl Language {
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
}

pub fn language_by_name(_name: &str) -> Option<Arc<Language>> {
    None
}

pub fn language_by_filename(_path: &Path) -> Option<Arc<Language>> {
    None
}
