use std::path::PathBuf;

use crate::{appearance::Appearance, terminal::view::InlineBannerId};
use warpui::{elements::Empty, Element};

#[derive(PartialEq, Clone)]
pub enum VisibilityState {
    Speedbump,
    Indexing,
}

#[derive(Clone, Copy, Debug)]
pub enum CodebaseIndexSpeedbumpBannerAction {
    ToggleAlwaysAllow,
    AllowIndexing,
    Close,
    ViewStatus,
    DismissForever,
}

pub struct CodebaseIndexSpeedbumpBannerState {
    pub id: InlineBannerId,
    pub always_allow_checked: bool,
    pub visibility_state: VisibilityState,
    pub repo_path: PathBuf,
}

impl CodebaseIndexSpeedbumpBannerState {
    #[cfg_attr(not(feature = "local_fs"), expect(unused))]
    pub fn new(id: InlineBannerId, repo_path: PathBuf) -> Self {
        Self {
            id,
            always_allow_checked: true,
            visibility_state: VisibilityState::Speedbump,
            repo_path,
        }
    }

    pub fn toggle_always_allow_checked(&mut self) {
        self.always_allow_checked = !self.always_allow_checked;
    }

    pub fn show_indexing_banner(&mut self) {
        self.visibility_state = VisibilityState::Indexing;
    }

    pub fn render_codebase_index_speedbump_banner(
        &self,
        _appearance: &Appearance,
    ) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
