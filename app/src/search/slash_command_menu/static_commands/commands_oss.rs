use std::{collections::HashMap, sync::LazyLock};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp_core::features::FeatureFlag;

use crate::search::slash_command_menu::{static_commands::Argument, StaticCommand};
use crate::ui_components::color_dot;

use super::Availability;

const fn unavailable_command() -> StaticCommand {
    StaticCommand {
        name: "",
        description: "",
        icon_path: "",
        availability: Availability::ALWAYS,
        auto_enter_ai_mode: false,
        argument: None,
    }
}

pub static AGENT: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub static CLOUD_AGENT: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub const ADD_MCP: StaticCommand = unavailable_command();
pub const PR_COMMENTS: StaticCommand = unavailable_command();
pub static CREATE_ENVIRONMENT: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub const CREATE_DOCKER_SANDBOX: StaticCommand = unavailable_command();
pub static CREATE_NEW_PROJECT: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub static EDIT_SKILL: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub static INVOKE_SKILL: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub static ADD_PROMPT: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub const ADD_RULE: StaticCommand = unavailable_command();
pub static EDIT: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub static FORK: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub static MOVE_TO_CLOUD: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub const OPEN_CODE_REVIEW: StaticCommand = unavailable_command();
pub const INDEX: StaticCommand = unavailable_command();
pub const INIT: StaticCommand = unavailable_command();
pub const OPEN_PROJECT_RULES: StaticCommand = unavailable_command();
pub const OPEN_MCP_SERVERS: StaticCommand = unavailable_command();
pub const OPEN_REPO: StaticCommand = unavailable_command();
pub const OPEN_RULES: StaticCommand = unavailable_command();
pub static NEW: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub static MODEL: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub static HOST: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub static HARNESS: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub static ENVIRONMENT: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub static PROFILE: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub const PLAN_NAME: &str = "";
pub static PLAN: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub const ORCHESTRATE_NAME: &str = "";
pub static ORCHESTRATE: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub static COMPACT: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub static COMPACT_AND: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub static QUEUE: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub static FORK_AND_COMPACT: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub const FORK_FROM: StaticCommand = unavailable_command();
pub static CONTINUE_LOCALLY: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub const USAGE: StaticCommand = unavailable_command();
pub const REMOTE_CONTROL: StaticCommand = unavailable_command();
pub const COST: StaticCommand = unavailable_command();
pub const CONVERSATIONS: StaticCommand = unavailable_command();
pub static PROMPTS: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);
pub const REWIND: StaticCommand = unavailable_command();
pub const EXPORT_TO_CLIPBOARD: StaticCommand = unavailable_command();
pub static EXPORT_TO_FILE: LazyLock<StaticCommand> = LazyLock::new(unavailable_command);

pub static RENAME_TAB: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/rename-tab",
    description: "Rename the current tab",
    icon_path: "bundled/svg/pencil-line.svg",
    availability: Availability::ALWAYS,
    auto_enter_ai_mode: false,
    argument: Some(Argument::required().with_hint_text("<tab name>")),
});

static SET_TAB_COLOR_HINT: LazyLock<String> = LazyLock::new(|| {
    let mut hint = String::from("<");
    for color in color_dot::TAB_COLOR_OPTIONS {
        hint.push_str(&color.to_string().to_ascii_lowercase());
        hint.push('|');
    }
    hint.push_str("none>");
    hint
});

pub static SET_TAB_COLOR: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/set-tab-color",
    description: "Set the color of the current tab",
    icon_path: "bundled/svg/ellipse.svg",
    availability: Availability::ALWAYS,
    auto_enter_ai_mode: false,
    argument: Some(Argument::required().with_hint_text(SET_TAB_COLOR_HINT.as_str())),
});

pub const OPEN_SETTINGS_FILE: StaticCommand = StaticCommand {
    name: "/open-settings-file",
    description: "Open settings file (TOML)",
    icon_path: "bundled/svg/file-code-02.svg",
    availability: Availability::LOCAL,
    auto_enter_ai_mode: false,
    argument: None,
};

pub const CHANGELOG: StaticCommand = StaticCommand {
    name: "/changelog",
    description: "Open the latest changelog",
    icon_path: "bundled/svg/book-open.svg",
    availability: Availability::ALWAYS,
    auto_enter_ai_mode: false,
    argument: None,
};

pub static FEEDBACK: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/feedback",
    description: "Send feedback",
    icon_path: "bundled/svg/feedback.svg",
    availability: Availability::ALWAYS,
    auto_enter_ai_mode: false,
    argument: Some(Argument::optional().with_execute_on_selection()),
});

/// If `query` starts with the given command `name` followed by a space,
/// returns the remainder of the query. Otherwise returns `None`.
pub fn strip_command_prefix(query: &str, name: &str) -> Option<String> {
    if name.is_empty() {
        return None;
    }

    query
        .strip_prefix(name)
        .and_then(|rest| rest.strip_prefix(' '))
        .map(|rest| rest.to_string())
}

pub static COMMAND_REGISTRY: LazyLock<Registry> = LazyLock::new(Registry::new);

/// A unique identifier for a static slash command.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct SlashCommandId(Uuid);

impl SlashCommandId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SlashCommandId {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Registry {
    commands: HashMap<SlashCommandId, StaticCommand>,
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        let mut commands = HashMap::new();
        for command in all_commands() {
            commands.insert(SlashCommandId::new(), command);
        }
        Self { commands }
    }

    pub fn all_commands_by_id(&self) -> impl Iterator<Item = (SlashCommandId, &StaticCommand)> {
        self.commands.iter().map(|(id, cmd)| (*id, cmd))
    }

    pub fn all_commands(&self) -> impl Iterator<Item = &StaticCommand> {
        self.commands.values()
    }

    pub fn get_command(&self, id: &SlashCommandId) -> Option<&StaticCommand> {
        self.commands.get(id)
    }

    pub fn get_command_with_name(&self, name: &str) -> Option<&StaticCommand> {
        self.commands.values().find(|command| command.name == name)
    }

    #[cfg(test)]
    pub fn get_command_id_with_name(&self, name: &str) -> Option<&SlashCommandId> {
        self.commands
            .iter()
            .find(|(_, command)| command.name == name)
            .map(|(id, _)| id)
    }
}

fn all_commands() -> Vec<StaticCommand> {
    let mut commands = vec![FEEDBACK.clone(), RENAME_TAB.clone(), SET_TAB_COLOR.clone()];

    if FeatureFlag::Changelog.is_enabled() {
        commands.push(CHANGELOG);
    }

    if FeatureFlag::SettingsFile.is_enabled() && cfg!(feature = "local_fs") {
        commands.push(OPEN_SETTINGS_FILE);
    }

    commands
}
