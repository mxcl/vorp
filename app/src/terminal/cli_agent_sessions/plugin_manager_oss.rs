use std::{fmt, io, path::PathBuf};

use async_trait::async_trait;

use crate::terminal::{shell::ShellType, CLIAgent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginModalKind {
    Install,
    Update,
}

pub(crate) struct PluginInstructionStep {
    pub description: &'static str,
    pub command: &'static str,
    pub executable: bool,
    pub link: Option<&'static str>,
}

pub(crate) struct PluginInstructions {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub steps: &'static [PluginInstructionStep],
    pub post_install_notes: &'static [&'static str],
}

pub(crate) struct PluginInstallError {
    pub message: String,
    pub log: String,
}

impl fmt::Display for PluginInstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl From<io::Error> for PluginInstallError {
    fn from(err: io::Error) -> Self {
        let msg = err.to_string();
        Self {
            message: msg.clone(),
            log: msg,
        }
    }
}

#[async_trait]
pub(crate) trait CliAgentPluginManager: Send + Sync {
    fn minimum_plugin_version(&self) -> &'static str;
    fn can_auto_install(&self) -> bool;
    fn is_installed(&self) -> bool {
        false
    }
    fn needs_update(&self) -> bool {
        false
    }
    async fn install(&self) -> Result<(), PluginInstallError>;
    async fn update(&self) -> Result<(), PluginInstallError>;
    fn install_success_message(&self) -> &'static str;
    fn update_success_message(&self) -> &'static str;
    fn install_instructions(&self) -> &'static PluginInstructions;
    fn supports_update(&self) -> bool {
        false
    }
    fn update_instructions(&self) -> &'static PluginInstructions;
    async fn install_platform_plugin(&self) -> Result<(), PluginInstallError> {
        Ok(())
    }
}

pub(crate) fn plugin_manager_for(_agent: CLIAgent) -> Option<Box<dyn CliAgentPluginManager>> {
    None
}

pub(crate) fn plugin_manager_for_with_shell(
    _agent: CLIAgent,
    _shell_path: Option<PathBuf>,
    _shell_type: Option<ShellType>,
    _path_env_var: Option<String>,
) -> Option<Box<dyn CliAgentPluginManager>> {
    None
}
