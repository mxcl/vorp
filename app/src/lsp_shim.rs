use std::{
    collections::HashMap,
    fmt,
    future::{self, Ready},
    path::{Path, PathBuf},
};

use anyhow::Result;
use async_trait::async_trait;
use instant::Instant;
use serde::{Deserialize, Serialize};
use warpui::{AppContext, Entity, ModelContext, ModelHandle, SingletonEntity};

pub mod notification {}
pub use lsp_types::{Position, Range};

pub mod types {
    use std::path::PathBuf;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FileLocation {
        pub path: PathBuf,
        pub location: Location,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Location {
        pub line: usize,
        pub column: usize,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Range {
        pub start: Location,
        pub end: Location,
    }

    impl From<lsp_types::Range> for Range {
        fn from(range: lsp_types::Range) -> Self {
            Self {
                start: Location {
                    line: range.start.line as usize,
                    column: range.start.character as usize,
                },
                end: Location {
                    line: range.end.line as usize,
                    column: range.end.character as usize,
                },
            }
        }
    }

    #[derive(Debug)]
    pub struct DefinitionLocation {
        pub origin: Option<Range>,
        pub target: FileLocation,
    }

    #[derive(Debug, Clone)]
    pub struct ReferenceLocation {
        pub file_path: PathBuf,
        pub range: Range,
    }

    pub struct TextEdit {
        pub range: Range,
        pub text: String,
    }

    pub struct DocumentVersion(i32);

    impl DocumentVersion {
        pub fn as_i32(&self) -> i32 {
            self.0
        }
    }

    impl From<usize> for DocumentVersion {
        fn from(version: usize) -> Self {
            Self(version as i32)
        }
    }

    #[derive(Debug)]
    pub struct TextDocumentContentChangeEvent {
        pub range: Option<Range>,
        pub text: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct WatchedFileChangeEvent {
        pub path: PathBuf,
        pub typ: lsp_types::FileChangeType,
    }

    #[derive(Debug, Clone)]
    pub struct HoverResult {
        pub contents: HoverContents,
        pub range: Option<Range>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum MarkupKind {
        PlainText,
        Markdown,
    }

    #[derive(Debug, Clone)]
    pub struct HoverContentSection {
        pub value: String,
        pub kind: MarkupKind,
    }

    #[derive(Debug, Clone)]
    pub struct HoverContents {
        pub sections: Vec<HoverContentSection>,
    }

    impl HoverContents {
        pub fn is_empty(&self) -> bool {
            self.sections.is_empty()
        }
    }
}

pub use types::{HoverContents, HoverResult, MarkupKind, ReferenceLocation};

pub mod supported_servers {
    pub use super::LSPServerType;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageId {
    Rust,
    Go,
    Python,
    TypeScript,
    TypeScriptReact,
    JavaScript,
    JavaScriptReact,
    C,
    Cpp,
}

impl LanguageId {
    pub fn from_path(path: &Path) -> Option<Self> {
        let extn = path.extension()?;
        match extn.to_str()? {
            "rs" => Some(Self::Rust),
            "go" => Some(Self::Go),
            "py" => Some(Self::Python),
            "ts" => Some(Self::TypeScript),
            "tsx" => Some(Self::TypeScriptReact),
            "js" | "mjs" | "cjs" => Some(Self::JavaScript),
            "jsx" => Some(Self::JavaScriptReact),
            "c" | "C" => Some(Self::C),
            "cc" | "cpp" | "cxx" | "h" | "H" | "hh" | "hpp" | "hxx" => Some(Self::Cpp),
            _ => None,
        }
    }

    pub fn server_type(&self) -> LSPServerType {
        match self {
            Self::Rust => LSPServerType::RustAnalyzer,
            Self::Go => LSPServerType::GoPls,
            Self::Python => LSPServerType::Pyright,
            Self::TypeScript | Self::TypeScriptReact | Self::JavaScript | Self::JavaScriptReact => {
                LSPServerType::TypeScriptLanguageServer
            }
            Self::C | Self::Cpp => LSPServerType::Clangd,
        }
    }

    fn lsp_language_identifier(&self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Go => "go",
            Self::Python => "python",
            Self::TypeScript => "typescript",
            Self::TypeScriptReact => "typescriptreact",
            Self::JavaScript => "javascript",
            Self::JavaScriptReact => "javascriptreact",
            Self::C => "c",
            Self::Cpp => "cpp",
        }
    }
}

#[derive(Debug, Clone)]
pub struct LspServerConfig {
    server_type: LSPServerType,
    initial_workspace: PathBuf,
    log_relative_path: Option<PathBuf>,
}

impl LspServerConfig {
    pub fn new(
        server_type: LSPServerType,
        initial_workspace: PathBuf,
        _path_env_var: Option<String>,
        _client_name: String,
        _client: std::sync::Arc<http_client::Client>,
    ) -> Self {
        Self {
            server_type,
            initial_workspace,
            log_relative_path: None,
        }
    }

    pub fn with_log_relative_path(mut self, log_relative_path: PathBuf) -> Self {
        self.log_relative_path = Some(log_relative_path);
        self
    }

    pub fn log_relative_path(&self) -> Option<&PathBuf> {
        self.log_relative_path.as_ref()
    }

    pub fn initial_workspace(&self) -> &Path {
        &self.initial_workspace
    }

    fn server_name(&self) -> String {
        self.server_type.binary_name().to_string()
    }

    fn languages(&self) -> Vec<LanguageId> {
        self.server_type.languages()
    }

    fn server_type(&self) -> LSPServerType {
        self.server_type
    }
}

#[derive(Clone, Debug)]
pub struct CommandBuilder {
    path_env_var: Option<String>,
}

impl CommandBuilder {
    pub fn new(path_env_var: Option<String>) -> Self {
        Self { path_env_var }
    }

    pub fn path_env_var(&self) -> Option<&str> {
        self.path_env_var.as_deref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LSPServerType {
    RustAnalyzer,
    GoPls,
    Pyright,
    TypeScriptLanguageServer,
    Clangd,
}

impl LSPServerType {
    pub fn all() -> impl Iterator<Item = LSPServerType> {
        [
            Self::RustAnalyzer,
            Self::GoPls,
            Self::Pyright,
            Self::TypeScriptLanguageServer,
            Self::Clangd,
        ]
        .into_iter()
    }

    pub fn binary_name(&self) -> &'static str {
        match self {
            Self::RustAnalyzer => "rust-analyzer",
            Self::GoPls => "gopls",
            Self::Pyright => "pyright-langserver",
            Self::TypeScriptLanguageServer => "typescript-language-server",
            Self::Clangd => "clangd",
        }
    }

    pub fn languages(&self) -> Vec<LanguageId> {
        match self {
            Self::RustAnalyzer => vec![LanguageId::Rust],
            Self::GoPls => vec![LanguageId::Go],
            Self::Pyright => vec![LanguageId::Python],
            Self::TypeScriptLanguageServer => vec![
                LanguageId::TypeScript,
                LanguageId::TypeScriptReact,
                LanguageId::JavaScript,
                LanguageId::JavaScriptReact,
            ],
            Self::Clangd => vec![LanguageId::C, LanguageId::Cpp],
        }
    }

    pub fn language_name(&self) -> String {
        match self {
            Self::TypeScriptLanguageServer => "TypeScript/JavaScript".to_string(),
            _ => self
                .languages()
                .iter()
                .map(|lang| {
                    let id = lang.lsp_language_identifier();
                    let mut chars = id.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join("/"),
        }
    }

    pub fn candidate(
        &self,
        _client: std::sync::Arc<http_client::Client>,
    ) -> Box<dyn LanguageServerCandidate> {
        Box::new(NoopLanguageServerCandidate)
    }
}

#[async_trait]
pub trait LanguageServerCandidate: Send + Sync {
    async fn should_suggest_for_repo(&self, _path: &Path, _executor: &CommandBuilder) -> bool {
        false
    }

    async fn is_installed_in_data_dir(&self, _executor: &CommandBuilder) -> bool {
        false
    }

    async fn is_installed_on_path(&self, _executor: &CommandBuilder) -> bool {
        false
    }

    async fn is_installed(&self, executor: &CommandBuilder) -> bool {
        self.is_installed_in_data_dir(executor).await || self.is_installed_on_path(executor).await
    }

    async fn install(
        &self,
        _metadata: LanguageServerMetadata,
        _executor: &CommandBuilder,
    ) -> Result<()> {
        anyhow::bail!("LSP is not available in this build")
    }

    async fn fetch_latest_server_metadata(&self) -> Result<LanguageServerMetadata> {
        anyhow::bail!("LSP is not available in this build")
    }
}

pub struct NoopLanguageServerCandidate;

#[async_trait]
impl LanguageServerCandidate for NoopLanguageServerCandidate {}

pub struct LanguageServerMetadata {
    pub version: String,
    pub url: Option<String>,
    pub digest: Option<String>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct LanguageServerId(usize);

#[derive(Debug)]
pub enum LspState {
    Stopped { manually_stopped: bool },
    Starting,
    Stopping { manually_stopped: bool },
    Available {},
    Failed { error: String },
}

impl LspState {
    pub fn name(&self) -> &str {
        match self {
            Self::Stopped { .. } => "stopped",
            Self::Starting => "starting",
            Self::Stopping { .. } => "stopping",
            Self::Available { .. } => "available",
            Self::Failed { .. } => "failed",
        }
    }

    pub fn can_auto_start(&self) -> bool {
        match self {
            Self::Stopped { manually_stopped } | Self::Stopping { manually_stopped } => {
                !manually_stopped
            }
            _ => true,
        }
    }
}

pub struct LspServerModel {
    id: LanguageServerId,
    config: LspServerConfig,
    state: LspState,
}

#[derive(Debug, Clone)]
pub struct BackgroundTaskInfo {
    pub task_token: String,
    pub message: Option<String>,
    pub finished: bool,
    pub updated_at: Instant,
}

impl BackgroundTaskInfo {
    pub fn to_display_message(&self) -> String {
        self.message
            .as_ref()
            .map(|message| format!("{} {message}", self.task_token))
            .unwrap_or_else(|| self.task_token.clone())
    }
}

#[derive(Debug, Clone)]
pub struct DocumentDiagnostics {
    pub diagnostics: Vec<lsp_types::Diagnostic>,
    pub version: Option<i32>,
    pub published_at: Instant,
}

#[derive(Debug)]
pub enum LspEvent {
    Starting,
    BackgroundTaskUpdated,
    Idle,
    Stopped,
    Failed(anyhow::Error),
    Started,
    DiagnosticsUpdated { path: PathBuf },
}

impl LspServerModel {
    fn new(config: LspServerConfig) -> Self {
        Self {
            id: LanguageServerId::default(),
            config,
            state: LspState::Stopped {
                manually_stopped: false,
            },
        }
    }

    pub fn id(&self) -> LanguageServerId {
        self.id
    }

    pub fn server_type(&self) -> LSPServerType {
        self.config.server_type()
    }

    pub fn server_name(&self) -> String {
        self.config.server_name()
    }

    pub fn state(&self) -> &LspState {
        &self.state
    }

    pub fn log_to_server_log(&self, _level: LspServerLogLevel, _message: impl Into<String>) {}

    pub fn latest_progress_update(&self) -> Option<&BackgroundTaskInfo> {
        None
    }

    pub fn is_ready_for_requests(&self) -> bool {
        false
    }

    pub fn has_started(&self) -> bool {
        false
    }

    pub fn has_pending_tasks(&self) -> bool {
        false
    }

    pub fn supports_language(&self, lang: &LanguageId) -> bool {
        self.config.languages().contains(lang)
    }

    pub fn initial_workspace(&self) -> &Path {
        self.config.initial_workspace()
    }

    pub fn can_auto_start(&self) -> bool {
        self.state.can_auto_start()
    }

    pub fn start(&mut self, _ctx: &mut ModelContext<Self>) -> Result<()> {
        self.state = LspState::Failed {
            error: "LSP is not available in this build".to_string(),
        };
        Ok(())
    }

    pub fn stop(&mut self, manually_stopped: bool, _ctx: &mut ModelContext<Self>) -> Result<()> {
        self.state = LspState::Stopped { manually_stopped };
        Ok(())
    }

    pub fn manual_start(&mut self, ctx: &mut ModelContext<Self>) -> Result<()> {
        self.start(ctx)
    }

    pub fn restart(&mut self, ctx: &mut ModelContext<Self>) {
        let _ = self.start(ctx);
    }

    pub fn document_is_open(&self, _path: &PathBuf) -> Result<bool> {
        Ok(false)
    }

    pub fn last_synced_version(&self, _path: &PathBuf) -> Result<Option<usize>> {
        Ok(None)
    }

    pub fn did_open_document(
        &self,
        _path: PathBuf,
        _content: String,
        _initial_version: usize,
    ) -> Result<Ready<Result<()>>> {
        Ok(future::ready(Ok(())))
    }

    pub fn did_close_document(&self, _path: PathBuf) -> Result<Ready<Result<()>>> {
        Ok(future::ready(Ok(())))
    }

    pub fn did_change_document(
        &self,
        _path: PathBuf,
        _version: types::DocumentVersion,
        _deltas: Vec<types::TextDocumentContentChangeEvent>,
    ) -> Result<Ready<Result<()>>> {
        Ok(future::ready(Ok(())))
    }

    pub fn did_change_watched_files(
        &self,
        _events: Vec<types::WatchedFileChangeEvent>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn goto_definition(
        &self,
        _path: PathBuf,
        _position: types::Location,
    ) -> Result<Ready<Result<Vec<types::DefinitionLocation>>>> {
        Ok(future::ready(Ok(Vec::new())))
    }

    pub fn format_document(
        &self,
        _path: PathBuf,
        _options: lsp_types::FormattingOptions,
    ) -> Result<Ready<Result<Option<Vec<types::TextEdit>>>>> {
        Ok(future::ready(Ok(None)))
    }

    pub fn hover(
        &self,
        _path: PathBuf,
        _position: types::Location,
    ) -> Result<Ready<Result<Option<types::HoverResult>>>> {
        Ok(future::ready(Ok(None)))
    }

    pub fn diagnostics_for_path(&self, _path: &Path) -> Result<Option<&DocumentDiagnostics>> {
        Ok(None)
    }

    pub fn find_references(
        &self,
        _path: PathBuf,
        _position: types::Location,
    ) -> Result<Ready<Result<Vec<types::ReferenceLocation>>>> {
        Ok(future::ready(Ok(Vec::new())))
    }
}

impl Entity for LspServerModel {
    type Event = LspEvent;
}

#[derive(Debug)]
pub enum LspManagerModelEvent {
    ServerStarted(PathBuf),
    ServerStopped(PathBuf),
    ServerRemoved {
        workspace_root: PathBuf,
        server_type: LSPServerType,
        server_id: LanguageServerId,
    },
}

#[derive(Default)]
pub struct LspManagerModel {
    servers: HashMap<PathBuf, Vec<ModelHandle<LspServerModel>>>,
}

impl LspManagerModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn workspace_roots(&self) -> impl Iterator<Item = &PathBuf> {
        self.servers.keys()
    }

    pub fn servers_for_workspace(&self, path: &Path) -> Option<&Vec<ModelHandle<LspServerModel>>> {
        self.servers.get(path)
    }

    pub fn server_registered(
        &self,
        _path: &Path,
        _server_type: LSPServerType,
        _ctx: &AppContext,
    ) -> bool {
        false
    }

    pub fn server_registered_and_started(
        &self,
        _path: &Path,
        _server_type: LSPServerType,
        _ctx: &AppContext,
    ) -> bool {
        false
    }

    pub fn server_for_path(
        &self,
        _path: &Path,
        _ctx: &AppContext,
    ) -> Option<ModelHandle<LspServerModel>> {
        None
    }

    pub fn maybe_register_external_file(&mut self, _path: &Path, _server_id: LanguageServerId) {}

    pub fn server_by_id(
        &self,
        _id: LanguageServerId,
        _ctx: &AppContext,
    ) -> Option<ModelHandle<LspServerModel>> {
        None
    }

    pub fn register(
        &mut self,
        path: PathBuf,
        config: LspServerConfig,
        ctx: &mut ModelContext<Self>,
    ) -> bool {
        let lsp = ctx.add_model(|_| LspServerModel::new(config));
        self.servers.entry(path).or_default().push(lsp);
        true
    }

    pub fn start_all(&mut self, _path: PathBuf, _ctx: &mut ModelContext<Self>) {}

    pub fn stop_all(&mut self, _path: PathBuf, _ctx: &mut ModelContext<Self>) {}

    pub fn remove_server(
        &mut self,
        workspace_root: &Path,
        server_type: LSPServerType,
        ctx: &mut ModelContext<Self>,
    ) {
        ctx.emit(LspManagerModelEvent::ServerRemoved {
            workspace_root: workspace_root.to_path_buf(),
            server_type,
            server_id: LanguageServerId::default(),
        });
    }

    pub fn terminate(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn lsp_model_for_path(&self, _path: &Path) -> Option<&[ModelHandle<LspServerModel>]> {
        None
    }
}

impl Entity for LspManagerModel {
    type Event = LspManagerModelEvent;
}

impl SingletonEntity for LspManagerModel {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LspServerLogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LspServerLogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = match self {
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        };
        f.write_str(level)
    }
}

pub fn init(app: &mut AppContext) {
    app.add_singleton_model(|_| LspManagerModel::new());
}
