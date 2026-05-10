use std::path::{Path, PathBuf};

use lsp::supported_servers::LSPServerType;
use warpui::{
    elements::{Empty, MouseStateHandle},
    AppContext, Element, Entity, ModelContext, ModelHandle, TypedActionView, View, ViewContext,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InitStepKind {
    Welcome = 0,
    CodebaseContext = 1,
    LanguageServers = 2,
    ProjectScopedRules = 3,
    CreateEnvironment = 4,
}

pub enum CodebaseIndexingResult {
    Accepted,
    Skipped,
}

pub enum LanguageServersResult {
    Accepted {
        enabled_servers: Vec<LSPServerType>,
        servers_to_install: Vec<LSPServerType>,
    },
    Skipped,
}

pub enum CreateEnvironmentResult {
    Created,
    Skipped,
}

pub enum InitActionResult {
    Welcome,
    CodebaseContext(CodebaseIndexingResult),
    ProjectScopedRules(ProjectScopedRulesResult),
    LanguageServers(LanguageServersResult),
    CreateEnvironment(CreateEnvironmentResult),
}

pub enum ProjectScopedRulesResult {
    LinkedFromExisting(String),
    GenerateNew {
        mouse_state: MouseStateHandle,
        button_disabled: bool,
    },
    AlreadyExists {
        button_disabled: bool,
    },
    Skipped,
}

#[derive(Debug, Clone)]
pub enum InitProjectBlockAction {
    Noop,
}

#[derive(Debug, Clone)]
pub enum InitProjectModelEvent {
    InsertStep(InitStepKind),
    StepCompleted(InitStepKind),
    Cancelled,
    InitCompleted,
    GenerateProjectRules,
    RegenerateProjectRules,
    ViewCodebaseContextStatus,
    LanguageServerInstalledAndEnabled,
    CreateEnvironment,
    EnvironmentCreated,
}

pub struct InitProjectModel {
    is_cancelled: bool,
    #[cfg(feature = "local_fs")]
    root_path: PathBuf,
    path_env_var: Option<String>,
}

impl InitProjectModel {
    pub fn new(
        pwd_path: PathBuf,
        path_env_var: Option<String>,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self {
            is_cancelled: false,
            #[cfg(feature = "local_fs")]
            root_path: pwd_path,
            path_env_var,
        }
    }

    pub fn start(&mut self, ctx: &mut ModelContext<Self>) {
        ctx.emit(InitProjectModelEvent::InitCompleted);
    }

    pub fn should_have_available_steps(_path: &Path, _ctx: &AppContext) -> bool {
        false
    }

    pub fn is_cancelled(&self) -> bool {
        self.is_cancelled
    }

    pub fn is_already_setup(&self) -> bool {
        true
    }

    pub fn is_completed(&self) -> bool {
        true
    }

    pub fn is_active(&self) -> bool {
        false
    }

    #[cfg(feature = "local_fs")]
    pub fn root_path(&self) -> &Path {
        &self.root_path
    }

    pub fn path_env_var(&self) -> Option<&String> {
        self.path_env_var.as_ref()
    }

    pub fn mark_step_completed(
        &mut self,
        kind: InitStepKind,
        _result: InitActionResult,
        ctx: &mut ModelContext<Self>,
    ) {
        ctx.emit(InitProjectModelEvent::StepCompleted(kind));
        ctx.emit(InitProjectModelEvent::InitCompleted);
    }

    pub fn mark_step_running(&mut self, _kind: InitStepKind, ctx: &mut ModelContext<Self>) {
        ctx.notify();
    }

    pub fn disable_regenerate_button(&mut self) {}

    pub fn cancel(&mut self, ctx: &mut ModelContext<Self>) {
        self.is_cancelled = true;
        ctx.emit(InitProjectModelEvent::Cancelled);
    }
}

impl Entity for InitProjectModel {
    type Event = InitProjectModelEvent;
}

pub struct InitStepBlock;

impl InitStepBlock {
    pub fn new(
        _step_kind: InitStepKind,
        _model: ModelHandle<InitProjectModel>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn try_steal_focus(&self, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for InitStepBlock {
    type Event = ();
}

impl TypedActionView for InitStepBlock {
    type Action = InitProjectBlockAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for InitStepBlock {
    fn ui_name() -> &'static str {
        "InitStepBlock"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
