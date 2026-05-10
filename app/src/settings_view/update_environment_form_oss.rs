use crate::{
    ai::cloud_environments::{AmbientAgentEnvironment, GithubRepo},
    server::ids::SyncId,
};
use warpui::{elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext};

pub fn init(_app: &mut AppContext) {}

#[derive(Clone, Debug, Default)]
pub struct EnvironmentFormValues {
    pub name: String,
    pub description: String,
    pub selected_repos: Vec<GithubRepo>,
    pub docker_image: String,
    pub setup_commands: Vec<String>,
}

impl EnvironmentFormValues {
    pub fn to_ambient_agent_environment(&self) -> AmbientAgentEnvironment {
        AmbientAgentEnvironment::new(
            self.name.trim().to_string(),
            Some(self.description.trim().to_string()).filter(|description| !description.is_empty()),
            self.selected_repos.clone(),
            self.docker_image.trim().to_string(),
            self.setup_commands
                .iter()
                .map(|command| command.trim())
                .filter(|command| !command.is_empty())
                .map(ToString::to_string)
                .collect(),
        )
    }

    pub fn is_valid(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug)]
pub enum EnvironmentFormInitArgs {
    Create,
    Edit {
        env_id: SyncId,
        initial_values: Box<EnvironmentFormValues>,
    },
}

#[derive(Clone, Debug)]
pub enum EnvironmentFormMode {
    Create,
    Edit { env_id: SyncId },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GithubAuthRedirectTarget {
    SettingsEnvironments,
    FocusCloudMode,
}

#[derive(Debug, Clone)]
pub enum UpdateEnvironmentFormEvent {
    Created {
        environment: AmbientAgentEnvironment,
        share_with_team: bool,
    },
    Updated {
        env_id: SyncId,
        environment: AmbientAgentEnvironment,
    },
    DeleteRequested {
        env_id: SyncId,
    },
    Cancelled,
}

#[derive(Debug, Clone)]
pub enum UpdateEnvironmentFormAction {
    Noop,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AuthSource {
    #[default]
    Settings,
    CloudSetup,
}

pub struct UpdateEnvironmentForm {
    github_auth_redirect_target: GithubAuthRedirectTarget,
    mode: EnvironmentFormMode,
}

impl UpdateEnvironmentForm {
    pub fn new(init_args: EnvironmentFormInitArgs, _ctx: &mut ViewContext<Self>) -> Self {
        let mode = match init_args {
            EnvironmentFormInitArgs::Create => EnvironmentFormMode::Create,
            EnvironmentFormInitArgs::Edit { env_id, .. } => EnvironmentFormMode::Edit { env_id },
        };
        Self {
            github_auth_redirect_target: GithubAuthRedirectTarget::SettingsEnvironments,
            mode,
        }
    }

    pub fn set_mode(&mut self, init_args: EnvironmentFormInitArgs, _ctx: &mut ViewContext<Self>) {
        self.mode = match init_args {
            EnvironmentFormInitArgs::Create => EnvironmentFormMode::Create,
            EnvironmentFormInitArgs::Edit { env_id, .. } => EnvironmentFormMode::Edit { env_id },
        };
    }

    pub fn set_github_auth_redirect_target(&mut self, target: GithubAuthRedirectTarget) {
        self.github_auth_redirect_target = target;
    }

    pub fn set_show_header(&mut self, _show_header: bool, _ctx: &mut ViewContext<Self>) {}

    pub fn set_should_handle_escape_from_editor(&mut self, _should_handle_escape: bool) {}

    pub fn set_auth_source(&mut self, _source: AuthSource) {}

    pub fn fetch_github_repos(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn start_github_auth(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn focus(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.focus_self();
    }
}

impl Entity for UpdateEnvironmentForm {
    type Event = UpdateEnvironmentFormEvent;
}

impl TypedActionView for UpdateEnvironmentForm {
    type Action = UpdateEnvironmentFormAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for UpdateEnvironmentForm {
    fn ui_name() -> &'static str {
        "UpdateEnvironmentForm"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
