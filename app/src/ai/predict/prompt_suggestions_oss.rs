use std::sync::LazyLock;

use warpui::{keymap::Keystroke, AppContext};

use crate::terminal::TerminalModel;

pub const ACCEPT_PROMPT_SUGGESTION_KEYBINDING: &str = "terminal:accept_prompt_suggestions";

pub static REJECT_PROMPT_SUGGESTION_KEYSTROKE: LazyLock<Keystroke> = LazyLock::new(|| Keystroke {
    ctrl: true,
    key: "c".to_owned(),
    ..Default::default()
});

pub fn is_accept_prompt_suggestion_bound_to_cmd_enter(_app: &AppContext) -> bool {
    false
}

pub fn is_accept_prompt_suggestion_bound_to_ctrl_enter(_app: &AppContext) -> bool {
    false
}

pub fn has_pending_code_or_unit_test_prompt_suggestion(
    _terminal_model: &TerminalModel,
    _app: &AppContext,
) -> bool {
    false
}
