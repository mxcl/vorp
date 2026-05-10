#[cfg(all(feature = "remote_server_runtime", not(target_family = "wasm")))]
use crate::remote_server::manager::RemoteServerManager;
#[cfg(all(feature = "remote_server_runtime", not(target_family = "wasm")))]
use crate::server::server_api::{ServerApiEvent, ServerApiProvider};
#[cfg(all(feature = "remote_server_runtime", not(target_family = "wasm")))]
use warpui::SingletonEntity;
// Re-export everything from the `remote_server` crate so existing
// `crate::remote_server::*` imports in `app` continue to work.
#[cfg(feature = "remote_server_runtime")]
pub use ::remote_server::*;

#[cfg(not(feature = "remote_server_runtime"))]
pub mod auth {
    use std::sync::Arc;

    use warpui::r#async::BoxFuture;

    type GetAuthTokenFn = dyn Fn() -> BoxFuture<'static, Option<String>> + Send + Sync;
    type RemoteServerIdentityKeyFn = dyn Fn() -> String + Send + Sync;

    #[derive(Clone)]
    pub struct RemoteServerAuthContext {
        get_auth_token: Arc<GetAuthTokenFn>,
        remote_server_identity_key: Arc<RemoteServerIdentityKeyFn>,
        user_id: String,
        user_email: String,
        crash_reporting_enabled: bool,
    }

    impl RemoteServerAuthContext {
        pub fn new(
            get_auth_token: impl Fn() -> BoxFuture<'static, Option<String>> + Send + Sync + 'static,
            remote_server_identity_key: impl Fn() -> String + Send + Sync + 'static,
            user_id: String,
            user_email: String,
            crash_reporting_enabled: bool,
        ) -> Self {
            Self {
                get_auth_token: Arc::new(get_auth_token),
                remote_server_identity_key: Arc::new(remote_server_identity_key),
                user_id,
                user_email,
                crash_reporting_enabled,
            }
        }

        pub fn get_auth_token(&self) -> BoxFuture<'static, Option<String>> {
            (self.get_auth_token)()
        }

        pub fn remote_server_identity_key(&self) -> String {
            (self.remote_server_identity_key)()
        }

        pub fn user_id(&self) -> &str {
            &self.user_id
        }

        pub fn user_email(&self) -> &str {
            &self.user_email
        }

        pub fn crash_reporting_enabled(&self) -> bool {
            self.crash_reporting_enabled
        }
    }
}

#[cfg(not(feature = "remote_server_runtime"))]
pub mod auth_context {
    use std::sync::Arc;

    use crate::auth::auth_state::AuthState;
    use crate::server::server_api::auth::AuthClient;

    use super::auth::RemoteServerAuthContext;

    pub fn server_api_auth_context(
        auth_state: Arc<AuthState>,
        _auth_client: Arc<dyn AuthClient>,
        crash_reporting_enabled: bool,
    ) -> RemoteServerAuthContext {
        let identity_auth_state = auth_state.clone();
        let user_id_auth_state = auth_state.clone();
        let user_email_auth_state = auth_state;
        let user_id = user_id_auth_state
            .user_id()
            .map(|uid| uid.as_string())
            .unwrap_or_default();
        let user_email = user_email_auth_state.user_email().unwrap_or_default();

        RemoteServerAuthContext::new(
            || Box::pin(async { None }),
            move || identity_auth_state.anonymous_id(),
            user_id,
            user_email,
            crash_reporting_enabled,
        )
    }
}

#[cfg(not(feature = "remote_server_runtime"))]
pub mod ssh_transport {
    use std::{fmt, path::PathBuf, sync::Arc};

    use super::auth::RemoteServerAuthContext;

    #[derive(Clone)]
    pub struct SshTransport {
        socket_path: PathBuf,
        _auth_context: Arc<RemoteServerAuthContext>,
    }

    impl fmt::Debug for SshTransport {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SshTransport")
                .field("socket_path", &self.socket_path)
                .finish_non_exhaustive()
        }
    }

    impl SshTransport {
        pub fn new(socket_path: PathBuf, auth_context: Arc<RemoteServerAuthContext>) -> Self {
            Self {
                socket_path,
                _auth_context: auth_context,
            }
        }

        pub fn socket_path(&self) -> &PathBuf {
            &self.socket_path
        }
    }
}

#[cfg(not(feature = "remote_server_runtime"))]
pub mod client {
    use anyhow::{anyhow, Result};
    use warp_core::SessionId;

    use super::proto::{ReadFileContextRequest, ReadFileContextResponse, RunCommandResponse};

    #[derive(Debug, Clone)]
    pub struct RemoteServerClient;

    impl RemoteServerClient {
        pub async fn run_command(
            &self,
            _session_id: SessionId,
            _command: String,
            _current_directory_path: Option<String>,
            _environment_variables: std::collections::HashMap<String, String>,
        ) -> Result<RunCommandResponse> {
            Err(anyhow!("remote server is not available in this build"))
        }

        pub async fn read_file_context(
            &self,
            _request: ReadFileContextRequest,
        ) -> Result<ReadFileContextResponse> {
            Err(anyhow!("remote server is not available in this build"))
        }

        pub fn update_preferences(&self, _crash_reporting_enabled: bool) {}
    }
}

#[cfg(not(feature = "remote_server_runtime"))]
pub mod setup {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct GlibcVersion {
        pub major: u32,
        pub minor: u32,
    }

    impl std::fmt::Display for GlibcVersion {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}.{}", self.major, self.minor)
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum RemoteLibc {
        Glibc(GlibcVersion),
        NonGlibc { name: String },
        Unknown,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum UnsupportedReason {
        GlibcTooOld {
            detected: GlibcVersion,
            required: GlibcVersion,
        },
        NonGlibc {
            name: String,
        },
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum PreinstallStatus {
        Supported,
        Unsupported { reason: UnsupportedReason },
        Unknown,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct PreinstallCheckResult {
        pub status: PreinstallStatus,
        pub libc: RemoteLibc,
        pub raw: String,
    }

    impl PreinstallCheckResult {
        pub fn is_supported(&self) -> bool {
            !matches!(self.status, PreinstallStatus::Unsupported { .. })
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum RemoteServerSetupState {
        Checking,
        Installing { progress_percent: Option<u8> },
        Updating,
        Initializing,
        Ready,
        Failed { error: String },
        Unsupported { reason: UnsupportedReason },
    }

    impl RemoteServerSetupState {
        pub fn is_ready(&self) -> bool {
            matches!(self, Self::Ready)
        }

        pub fn is_failed(&self) -> bool {
            matches!(self, Self::Failed { .. })
        }

        pub fn is_unsupported(&self) -> bool {
            matches!(self, Self::Unsupported { .. })
        }

        pub fn is_terminal(&self) -> bool {
            self.is_ready() || self.is_failed() || self.is_unsupported()
        }

        pub fn is_in_progress(&self) -> bool {
            matches!(
                self,
                Self::Checking | Self::Installing { .. } | Self::Updating | Self::Initializing
            )
        }

        pub fn is_connecting(&self) -> bool {
            matches!(
                self,
                Self::Installing { .. } | Self::Updating | Self::Initializing
            )
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct RemotePlatform {
        pub os: RemoteOs,
        pub arch: RemoteArch,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum RemoteOs {
        Linux,
        MacOs,
    }

    impl RemoteOs {
        pub fn as_str(&self) -> &'static str {
            match self {
                Self::Linux => "linux",
                Self::MacOs => "macos",
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum RemoteArch {
        X86_64,
        Aarch64,
    }

    impl RemoteArch {
        pub fn as_str(&self) -> &'static str {
            match self {
                Self::X86_64 => "x86_64",
                Self::Aarch64 => "aarch64",
            }
        }
    }
}

#[cfg(not(feature = "remote_server_runtime"))]
pub mod proto {
    #[derive(Default)]
    pub struct RunCommandResponse {
        pub result: Option<run_command_response::Result>,
    }

    pub mod run_command_response {
        #[derive(Clone)]
        pub enum Result {
            Success(super::RunCommandSuccess),
            Error(super::RunCommandError),
        }
    }

    #[derive(Clone)]
    pub struct RunCommandSuccess {
        pub stdout: Vec<u8>,
        pub stderr: Vec<u8>,
        pub exit_code: Option<i32>,
    }

    #[derive(Clone)]
    pub struct RunCommandError {
        pub message: String,
    }

    impl RunCommandError {
        pub fn code(&self) -> i32 {
            0
        }
    }

    #[derive(Default)]
    pub struct ReadFileContextRequest {
        pub files: Vec<ReadFileContextFile>,
        pub max_file_bytes: Option<u32>,
        pub max_batch_bytes: Option<u32>,
    }

    pub struct ReadFileContextFile {
        pub path: String,
        pub line_ranges: Vec<LineRange>,
    }

    pub struct LineRange {
        pub start: u32,
        pub end: u32,
    }

    #[derive(Default)]
    pub struct ReadFileContextResponse {
        pub file_contexts: Vec<FileContextProto>,
        pub failed_files: Vec<FailedFileRead>,
    }

    pub struct FailedFileRead {
        pub path: String,
        pub error: Option<FileOperationError>,
    }

    pub struct FileOperationError {
        pub message: String,
    }

    pub struct FileContextProto {
        pub file_name: String,
        pub content: Option<file_context_proto::Content>,
        pub line_range_start: Option<u32>,
        pub line_range_end: Option<u32>,
        pub last_modified_epoch_millis: Option<u64>,
        pub line_count: u32,
    }

    pub mod file_context_proto {
        pub enum Content {
            TextContent(String),
            BinaryContent(Vec<u8>),
        }
    }
}

#[cfg(not(feature = "remote_server_runtime"))]
pub mod manager {
    use std::{collections::HashSet, sync::Arc};

    use repo_metadata::RepoMetadataUpdate;
    use serde::Serialize;
    use warp_core::{HostId, SessionId};
    use warpui::{Entity, ModelContext, SingletonEntity};

    use super::{
        client::RemoteServerClient,
        setup::{PreinstallCheckResult, RemotePlatform, RemoteServerSetupState, UnsupportedReason},
    };

    #[derive(Clone, Copy, Debug, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum RemoteServerInitPhase {
        Connect,
        Initialize,
    }

    #[derive(Clone, Copy, Debug, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum RemoteServerOperation {
        NavigateToDirectory,
        LoadRepoMetadataDirectory,
    }

    #[derive(Clone, Copy, Debug, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum RemoteServerErrorKind {
        Timeout,
        Disconnected,
        ServerError,
        Other,
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct RemoteServerExitStatus {
        pub code: Option<i32>,
        pub signal_killed: bool,
    }

    #[derive(Clone, Debug)]
    pub enum RemoteServerManagerEvent {
        SessionConnecting {
            session_id: SessionId,
        },
        SessionConnected {
            session_id: SessionId,
            host_id: HostId,
        },
        SessionConnectionFailed {
            session_id: SessionId,
            phase: RemoteServerInitPhase,
            error: String,
        },
        SessionDisconnected {
            session_id: SessionId,
            host_id: HostId,
            exit_status: Option<RemoteServerExitStatus>,
        },
        SessionReconnected {
            session_id: SessionId,
            host_id: HostId,
            attempt: u32,
            client: Arc<RemoteServerClient>,
        },
        SessionDeregistered {
            session_id: SessionId,
        },
        HostConnected {
            host_id: HostId,
        },
        HostDisconnected {
            host_id: HostId,
        },
        NavigatedToDirectory {
            session_id: SessionId,
            host_id: HostId,
            indexed_path: String,
            is_git: bool,
        },
        RepoMetadataSnapshot {
            host_id: HostId,
            update: RepoMetadataUpdate,
        },
        RepoMetadataUpdated {
            host_id: HostId,
            update: RepoMetadataUpdate,
        },
        RepoMetadataDirectoryLoaded {
            host_id: HostId,
            update: RepoMetadataUpdate,
        },
        SetupStateChanged {
            session_id: SessionId,
            state: RemoteServerSetupState,
        },
        BinaryCheckComplete {
            session_id: SessionId,
            result: Result<bool, String>,
            remote_platform: Option<RemotePlatform>,
            preinstall_check: Option<PreinstallCheckResult>,
            has_old_binary: bool,
        },
        BinaryInstallComplete {
            session_id: SessionId,
            result: Result<(), String>,
        },
        ClientRequestFailed {
            session_id: SessionId,
            operation: RemoteServerOperation,
            error_kind: RemoteServerErrorKind,
        },
        ServerMessageDecodingError {
            session_id: SessionId,
        },
    }

    impl RemoteServerManagerEvent {
        pub fn session_id(&self) -> Option<SessionId> {
            match self {
                Self::SessionConnecting { session_id }
                | Self::SessionConnected { session_id, .. }
                | Self::SessionConnectionFailed { session_id, .. }
                | Self::SessionDisconnected { session_id, .. }
                | Self::SessionReconnected { session_id, .. }
                | Self::SessionDeregistered { session_id }
                | Self::NavigatedToDirectory { session_id, .. }
                | Self::SetupStateChanged { session_id, .. }
                | Self::BinaryCheckComplete { session_id, .. }
                | Self::BinaryInstallComplete { session_id, .. }
                | Self::ClientRequestFailed { session_id, .. }
                | Self::ServerMessageDecodingError { session_id } => Some(*session_id),
                Self::HostConnected { .. }
                | Self::HostDisconnected { .. }
                | Self::RepoMetadataSnapshot { .. }
                | Self::RepoMetadataUpdated { .. }
                | Self::RepoMetadataDirectoryLoaded { .. } => None,
            }
        }
    }

    pub struct RemoteServerManager;

    impl Entity for RemoteServerManager {
        type Event = RemoteServerManagerEvent;
    }

    impl SingletonEntity for RemoteServerManager {}

    impl RemoteServerManager {
        pub fn new(_: &mut ModelContext<Self>) -> Self {
            Self
        }

        pub fn client_for_host(&self, _: &HostId) -> Option<&Arc<RemoteServerClient>> {
            None
        }

        pub fn client_for_session(&self, _: SessionId) -> Option<&Arc<RemoteServerClient>> {
            None
        }

        pub fn all_connected_clients(&self) -> impl Iterator<Item = &Arc<RemoteServerClient>> {
            std::iter::empty()
        }

        pub fn rotate_auth_token(&mut self, _: String) {}

        pub fn is_session_potentially_active(&self, _: SessionId) -> bool {
            false
        }

        pub fn navigate_to_directory(
            &mut self,
            _: SessionId,
            _: String,
            _: &mut ModelContext<Self>,
        ) {
        }

        pub fn sessions_for_host(&self, _: &HostId) -> Option<&HashSet<SessionId>> {
            None
        }

        pub fn load_remote_repo_metadata_directory(
            &mut self,
            _: SessionId,
            _: String,
            _: String,
            _: &mut ModelContext<Self>,
        ) {
        }

        pub fn platform_for_session(&self, _: SessionId) -> Option<&RemotePlatform> {
            None
        }

        pub fn host_id_for_session(&self, _: SessionId) -> Option<&HostId> {
            None
        }

        pub fn deregister_session(&mut self, _: SessionId, _: &mut ModelContext<Self>) {}

        pub fn notify_session_bootstrapped(&mut self, _: SessionId, _: &str, _: Option<&str>) {}

        pub fn mark_setup_unsupported(
            &mut self,
            _: SessionId,
            _: UnsupportedReason,
            _: &mut ModelContext<Self>,
        ) {
        }

        pub fn check_binary<T>(
            &mut self,
            session_id: SessionId,
            _: T,
            ctx: &mut ModelContext<Self>,
        ) {
            ctx.emit(RemoteServerManagerEvent::BinaryCheckComplete {
                session_id,
                result: Err("remote server is not available in this build".to_string()),
                remote_platform: None,
                preinstall_check: None,
                has_old_binary: false,
            });
        }

        pub fn install_binary<T>(
            &mut self,
            session_id: SessionId,
            _: T,
            _: bool,
            ctx: &mut ModelContext<Self>,
        ) {
            ctx.emit(RemoteServerManagerEvent::BinaryInstallComplete {
                session_id,
                result: Err("remote server is not available in this build".to_string()),
            });
        }

        pub fn connect_session<T>(
            &mut self,
            session_id: SessionId,
            _: T,
            _: Arc<super::auth::RemoteServerAuthContext>,
            ctx: &mut ModelContext<Self>,
        ) {
            ctx.emit(RemoteServerManagerEvent::SessionConnectionFailed {
                session_id,
                phase: RemoteServerInitPhase::Connect,
                error: "remote server is not available in this build".to_string(),
            });
        }
    }
}

#[cfg(all(feature = "remote_server_runtime", not(target_family = "wasm")))]
pub mod auth_context;
#[cfg(all(feature = "remote_server_runtime", not(target_family = "wasm")))]
pub mod server_model;
#[cfg(all(feature = "remote_server_runtime", not(target_family = "wasm")))]
pub mod ssh_transport;
#[cfg(all(feature = "remote_server_runtime", unix))]
pub mod unix;

#[cfg(all(not(feature = "remote_server_runtime"), unix))]
pub mod unix {
    pub(crate) fn launch_daemon(_identity_key: &str, _ctx: &mut warpui::AppContext) {
        log::error!("remote-server-daemon is not available in this build");
    }
}

/// Run the `remote-server-proxy` subcommand.
#[cfg(all(feature = "remote_server_runtime", unix))]
pub fn run_proxy(identity_key: String) -> anyhow::Result<()> {
    unix::proxy::run(&identity_key)
}

#[cfg(not(all(feature = "remote_server_runtime", unix)))]
pub fn run_proxy(_identity_key: String) -> anyhow::Result<()> {
    anyhow::bail!("remote-server-proxy is not supported on this platform")
}

/// Run the `remote-server-daemon` subcommand.
#[cfg(all(feature = "remote_server_runtime", unix))]
pub fn run_daemon(identity_key: String) -> anyhow::Result<()> {
    unix::run_daemon(identity_key)
}

#[cfg(not(all(feature = "remote_server_runtime", unix)))]
pub fn run_daemon(_identity_key: String) -> anyhow::Result<()> {
    anyhow::bail!("remote-server-daemon is not supported on this platform")
}

/// Forwards app auth-token rotation and privacy preference change events
/// to the remote-server manager.
#[cfg(all(feature = "remote_server_runtime", not(target_family = "wasm")))]
pub fn wire_auth_token_rotation(ctx: &mut warpui::AppContext) {
    let server_api = ServerApiProvider::handle(ctx);
    let manager = RemoteServerManager::handle(ctx);
    ctx.subscribe_to_model(&server_api, move |_, event, ctx| {
        if let ServerApiEvent::AccessTokenRefreshed { token } = event {
            manager.update(ctx, |manager, _| {
                manager.rotate_auth_token(token.clone());
            });
        }
    });

    // Forward crash reporting preference changes to all connected daemons.
    use crate::settings::{PrivacySettings, PrivacySettingsChangedEvent};
    let privacy_settings = PrivacySettings::handle(ctx);
    let manager = RemoteServerManager::handle(ctx);
    ctx.subscribe_to_model(&privacy_settings, move |_, event, ctx| {
        if let &PrivacySettingsChangedEvent::UpdateIsCrashReportingEnabled { new_value, .. } = event
        {
            for client in manager.as_ref(ctx).all_connected_clients() {
                client.update_preferences(new_value);
            }
        }
    });
}

#[cfg(not(all(feature = "remote_server_runtime", not(target_family = "wasm"))))]
pub fn wire_auth_token_rotation(_ctx: &mut warpui::AppContext) {}
