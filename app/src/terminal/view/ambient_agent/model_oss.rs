use instant::Instant;
use session_sharing_protocol::common::SessionId;
use warp_cli::agent::Harness;
use warpui::{AppContext, Entity, EntityId, ModelContext};

use crate::ai::agent::conversation::AIConversationId;
use crate::ai::agent::UserQueryMode;
use crate::ai::ambient_agents::AmbientAgentTaskId;
#[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
use crate::ai::blocklist::handoff::touched_repos::TouchedWorkspace;
use crate::server::ids::SyncId;
#[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
use crate::server::server_api::ai::InitialSnapshotToken;
use crate::server::server_api::ai::{AgentConfigSnapshot, AttachmentInput, SpawnAgentRequest};
use crate::terminal::view::ambient_agent::{
    AmbientAgentProgressUIState, SetupCommandGroupId, SetupCommandState,
};
use crate::terminal::CLIAgent;

const OSS_DISABLED_MESSAGE: &str = "Cloud agents are disabled in OSS builds";

#[derive(Debug, Clone)]
pub struct AgentProgress {
    pub spawned_at: Instant,
    pub claimed_at: Option<Instant>,
    pub harness_started_at: Option<Instant>,
    pub stopped_at: Option<Instant>,
}

impl AgentProgress {
    fn new_stopped() -> Self {
        let now = Instant::now();
        Self {
            spawned_at: now,
            claimed_at: None,
            harness_started_at: None,
            stopped_at: Some(now),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionStartupKind {
    InitialRun,
    Followup,
}

#[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HandoffSubmissionState {
    #[default]
    Idle,
    Starting,
}

#[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SnapshotUploadStatus {
    #[default]
    Pending,
    SkippedEmptyWorkspace,
    Uploaded(InitialSnapshotToken),
    Failed(String),
}

#[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
impl SnapshotUploadStatus {
    fn is_settled(&self) -> bool {
        matches!(self, Self::Uploaded(_) | Self::SkippedEmptyWorkspace)
    }
}

#[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
#[derive(Debug, Clone)]
pub(crate) struct PendingHandoff {
    pub(crate) forked_conversation_id: String,
    pub(crate) touched_workspace: Option<TouchedWorkspace>,
    pub(crate) snapshot_upload: SnapshotUploadStatus,
    pub(crate) submission_state: HandoffSubmissionState,
}

#[derive(Debug, Clone)]
pub enum Status {
    Setup,
    Composing,
    WaitingForSession {
        progress: AgentProgress,
        kind: SessionStartupKind,
    },
    AgentRunning,
    Failed {
        progress: AgentProgress,
        error_message: String,
    },
    NeedsGithubAuth {
        progress: AgentProgress,
        error_message: String,
        auth_url: String,
    },
    Cancelled {
        progress: AgentProgress,
    },
}

pub struct AmbientAgentViewModel {
    status: Status,
    request: Option<SpawnAgentRequest>,
    terminal_view_id: EntityId,
    environment_id: Option<SyncId>,
    pub ui_state: AmbientAgentProgressUIState,
    setup_commands_state: SetupCommandState,
    task_id: Option<AmbientAgentTaskId>,
    conversation_id: Option<AIConversationId>,
    harness: Harness,
    worker_host: Option<String>,
    harness_command_started: bool,
    active_execution_session_id: Option<SessionId>,
    last_ended_execution_session_id: Option<SessionId>,
    pending_followup_prompt: Option<String>,
    #[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
    pending_handoff: Option<PendingHandoff>,
}

impl AmbientAgentViewModel {
    pub fn new(terminal_view_id: EntityId, ctx: &mut ModelContext<Self>) -> Self {
        Self {
            status: Status::Composing,
            request: None,
            terminal_view_id,
            environment_id: None,
            ui_state: AmbientAgentProgressUIState::new(ctx),
            setup_commands_state: Default::default(),
            task_id: None,
            conversation_id: None,
            harness: Harness::default(),
            worker_host: None,
            harness_command_started: false,
            active_execution_session_id: None,
            last_ended_execution_session_id: None,
            pending_followup_prompt: None,
            #[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
            pending_handoff: None,
        }
    }

    pub fn request(&self) -> Option<&SpawnAgentRequest> {
        self.request.as_ref()
    }

    pub fn setup_command_state(&self) -> &SetupCommandState {
        &self.setup_commands_state
    }

    pub fn setup_command_state_mut(&mut self) -> &mut SetupCommandState {
        &mut self.setup_commands_state
    }

    pub(super) fn start_new_setup_command_group(&mut self, ctx: &mut ModelContext<Self>) {
        self.setup_commands_state.start_new_group();
        self.harness_command_started = false;
        ctx.emit(AmbientAgentViewModelEvent::UpdatedSetupCommandVisibility);
    }

    pub(super) fn finish_setup_command_group(
        &mut self,
        group_id: SetupCommandGroupId,
        ctx: &mut ModelContext<Self>,
    ) {
        if self.setup_commands_state.is_running(group_id) {
            self.setup_commands_state.finish_group(group_id);
            ctx.emit(AmbientAgentViewModelEvent::UpdatedSetupCommandVisibility);
        }
    }

    pub(super) fn set_setup_command_group_visibility(
        &mut self,
        group_id: SetupCommandGroupId,
        is_visible: bool,
        ctx: &mut ModelContext<Self>,
    ) {
        if is_visible != self.setup_commands_state.should_expand(group_id) {
            self.setup_commands_state
                .set_should_expand(group_id, is_visible);
            ctx.emit(AmbientAgentViewModelEvent::UpdatedSetupCommandVisibility);
        }
    }

    pub(super) fn set_setup_command_visibility(
        &mut self,
        is_visible: bool,
        ctx: &mut ModelContext<Self>,
    ) {
        let group_id = self.setup_commands_state.current_group_id();
        self.set_setup_command_group_visibility(group_id, is_visible, ctx);
    }

    pub fn agent_progress(&self) -> Option<&AgentProgress> {
        match &self.status {
            Status::WaitingForSession { progress, .. }
            | Status::Failed { progress, .. }
            | Status::NeedsGithubAuth { progress, .. }
            | Status::Cancelled { progress } => Some(progress),
            _ => None,
        }
    }

    pub fn selected_environment_id(&self) -> Option<&SyncId> {
        self.environment_id.as_ref()
    }

    pub fn selected_harness(&self) -> Harness {
        if self.is_local_to_cloud_handoff() {
            Harness::Oz
        } else {
            self.harness
        }
    }

    pub fn set_harness(&mut self, harness: Harness, ctx: &mut ModelContext<Self>) {
        let harness = if self.is_local_to_cloud_handoff() {
            Harness::Oz
        } else {
            harness
        };
        if self.harness != harness {
            self.harness = harness;
            ctx.emit(AmbientAgentViewModelEvent::HarnessSelected);
        }
    }

    pub fn set_worker_host(&mut self, worker_host: Option<String>) {
        self.worker_host = worker_host;
    }

    pub(super) fn is_third_party_harness(&self) -> bool {
        self.selected_harness() != Harness::Oz
    }

    pub fn selected_third_party_cli_agent(&self) -> Option<CLIAgent> {
        CLIAgent::from_harness(self.selected_harness())
    }

    pub(crate) fn is_local_to_cloud_handoff(&self) -> bool {
        #[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
        {
            self.pending_handoff.is_some()
        }
        #[cfg(not(all(feature = "local_fs", not(target_family = "wasm"))))]
        {
            false
        }
    }

    pub(crate) fn is_handoff_ready_to_submit(&self) -> bool {
        #[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
        {
            let Some(handoff) = self.pending_handoff.as_ref() else {
                return false;
            };
            handoff.touched_workspace.is_some()
                && handoff.snapshot_upload.is_settled()
                && matches!(handoff.submission_state, HandoffSubmissionState::Idle)
        }
        #[cfg(not(all(feature = "local_fs", not(target_family = "wasm"))))]
        {
            false
        }
    }

    #[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
    pub(crate) fn set_pending_handoff(
        &mut self,
        pending: Option<PendingHandoff>,
        ctx: &mut ModelContext<Self>,
    ) {
        let previous_harness = self.selected_harness();
        self.pending_handoff = pending;
        if self.selected_harness() != previous_harness {
            ctx.emit(AmbientAgentViewModelEvent::HarnessSelected);
        }
        ctx.emit(AmbientAgentViewModelEvent::PendingHandoffChanged);
    }

    #[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
    pub(crate) fn set_pending_handoff_workspace(
        &mut self,
        workspace: TouchedWorkspace,
        ctx: &mut ModelContext<Self>,
    ) {
        if let Some(handoff) = self.pending_handoff.as_mut() {
            handoff.touched_workspace = Some(workspace);
            ctx.emit(AmbientAgentViewModelEvent::PendingHandoffChanged);
        }
    }

    #[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
    pub(crate) fn set_pending_handoff_snapshot_upload(
        &mut self,
        snapshot_upload: SnapshotUploadStatus,
        ctx: &mut ModelContext<Self>,
    ) {
        if let Some(handoff) = self.pending_handoff.as_mut() {
            handoff.snapshot_upload = snapshot_upload;
            ctx.emit(AmbientAgentViewModelEvent::PendingHandoffChanged);
        }
    }

    #[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
    pub(crate) fn record_handoff_snapshot_upload_failed(
        &mut self,
        error_message: String,
        ctx: &mut ModelContext<Self>,
    ) {
        self.set_pending_handoff_snapshot_upload(
            SnapshotUploadStatus::Failed(error_message.clone()),
            ctx,
        );
        ctx.emit(AmbientAgentViewModelEvent::HandoffSnapshotUploadFailed { error_message });
    }

    pub(super) fn harness_command_started(&self) -> bool {
        self.harness_command_started
    }

    pub(super) fn mark_harness_command_started(&mut self, ctx: &mut ModelContext<Self>) {
        if !self.harness_command_started {
            self.harness_command_started = true;
            ctx.emit(AmbientAgentViewModelEvent::HarnessCommandStarted);
        }
    }

    pub fn set_environment_id(
        &mut self,
        environment_id: Option<SyncId>,
        ctx: &mut ModelContext<Self>,
    ) {
        self.environment_id = environment_id;
        ctx.emit(AmbientAgentViewModelEvent::EnvironmentSelected);
    }

    pub fn is_ambient_agent(&self) -> bool {
        true
    }

    pub fn task_id(&self) -> Option<AmbientAgentTaskId> {
        self.task_id
    }

    pub fn is_in_setup(&self) -> bool {
        matches!(self.status, Status::Setup)
    }

    pub fn is_configuring_ambient_agent(&self) -> bool {
        matches!(self.status, Status::Composing)
    }

    pub fn is_waiting_for_session(&self) -> bool {
        matches!(self.status, Status::WaitingForSession { .. })
    }

    pub fn is_failed(&self) -> bool {
        matches!(self.status, Status::Failed { .. })
    }

    pub fn is_cancelled(&self) -> bool {
        matches!(self.status, Status::Cancelled { .. })
    }

    pub fn is_needs_github_auth(&self) -> bool {
        matches!(self.status, Status::NeedsGithubAuth { .. })
    }

    pub fn is_agent_running(&self) -> bool {
        matches!(self.status, Status::AgentRunning)
    }

    pub fn should_show_status_footer(&self) -> bool {
        self.is_waiting_for_session()
            || self.is_failed()
            || self.is_needs_github_auth()
            || self.is_cancelled()
    }

    pub fn error_message(&self) -> Option<&str> {
        match &self.status {
            Status::Failed { error_message, .. } => Some(error_message),
            _ => None,
        }
    }

    pub fn github_auth_url(&self) -> Option<&str> {
        None
    }

    pub fn github_auth_error_message(&self) -> Option<&str> {
        None
    }

    pub fn enter_setup(&mut self, ctx: &mut ModelContext<Self>) {
        self.status = Status::Setup;
        ctx.emit(AmbientAgentViewModelEvent::EnteredSetupState);
    }

    pub fn enter_composing_from_setup(&mut self, ctx: &mut ModelContext<Self>) {
        self.status = Status::Composing;
        ctx.emit(AmbientAgentViewModelEvent::EnteredComposingState);
    }

    pub fn enter_viewing_existing_session(
        &mut self,
        task_id: AmbientAgentTaskId,
        ctx: &mut ModelContext<Self>,
    ) {
        self.task_id = Some(task_id);
        self.status = Status::Failed {
            progress: AgentProgress::new_stopped(),
            error_message: OSS_DISABLED_MESSAGE.to_string(),
        };
        ctx.emit(AmbientAgentViewModelEvent::Failed {
            error_message: OSS_DISABLED_MESSAGE.to_string(),
        });
    }

    pub fn attach_followup_session(&mut self, session_id: SessionId, ctx: &mut ModelContext<Self>) {
        self.pending_followup_prompt = None;
        self.active_execution_session_id = Some(session_id);
        self.last_ended_execution_session_id = None;
        self.status = Status::AgentRunning;
        ctx.emit(AmbientAgentViewModelEvent::FollowupSessionReady { session_id });
    }

    pub fn record_ambient_execution_ended(&mut self, session_id: SessionId) {
        if self.active_execution_session_id.as_ref() == Some(&session_id) {
            self.active_execution_session_id = None;
        }
        self.last_ended_execution_session_id = Some(session_id);
    }

    pub fn submit_cloud_followup(&mut self, prompt: String, ctx: &mut ModelContext<Self>) {
        self.pending_followup_prompt = Some(prompt);
        self.fail_disabled(ctx);
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn pending_followup_prompt(&self) -> Option<&str> {
        self.pending_followup_prompt.as_deref()
    }

    pub fn should_show_followup_progress(&self) -> bool {
        self.pending_followup_prompt.is_some()
            && matches!(
                self.status,
                Status::WaitingForSession { .. }
                    | Status::Failed { .. }
                    | Status::NeedsGithubAuth { .. }
                    | Status::Cancelled { .. }
            )
    }

    pub fn reset_for_new_cloud_prompt(&mut self, ctx: &mut ModelContext<Self>) {
        self.status = Status::Composing;
        self.environment_id = None;
        self.task_id = None;
        self.conversation_id = None;
        self.harness_command_started = false;
        self.active_execution_session_id = None;
        self.last_ended_execution_session_id = None;
        self.pending_followup_prompt = None;
        self.setup_commands_state = Default::default();
        ctx.notify();
    }

    pub fn set_conversation_id(&mut self, id: Option<AIConversationId>) {
        self.conversation_id = id;
    }

    pub(crate) fn build_default_spawn_config(&self, _ctx: &AppContext) -> AgentConfigSnapshot {
        AgentConfigSnapshot {
            environment_id: self.environment_id.as_ref().map(ToString::to_string),
            worker_host: self.worker_host.clone(),
            ..Default::default()
        }
    }

    pub fn spawn_agent(
        &mut self,
        prompt: String,
        attachments: Vec<AttachmentInput>,
        ctx: &mut ModelContext<Self>,
    ) {
        let request = SpawnAgentRequest {
            prompt,
            mode: UserQueryMode::Normal,
            config: Some(self.build_default_spawn_config(ctx)),
            title: None,
            team: None,
            skill: None,
            attachments,
            interactive: None,
            parent_run_id: None,
            runtime_skills: vec![],
            referenced_attachments: vec![],
            conversation_id: None,
            initial_snapshot_token: None,
        };
        self.spawn_agent_with_request(request, ctx);
    }

    pub fn spawn_agent_with_request(
        &mut self,
        request: SpawnAgentRequest,
        ctx: &mut ModelContext<Self>,
    ) {
        self.request = Some(request);
        self.fail_disabled(ctx);
    }

    #[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
    pub(crate) fn submit_handoff(
        &mut self,
        prompt: String,
        attachments: Vec<AttachmentInput>,
        ctx: &mut ModelContext<Self>,
    ) {
        let mut request = SpawnAgentRequest {
            prompt,
            mode: UserQueryMode::Normal,
            config: Some(self.build_default_spawn_config(ctx)),
            title: None,
            team: None,
            skill: None,
            attachments,
            interactive: None,
            parent_run_id: None,
            runtime_skills: vec![],
            referenced_attachments: vec![],
            conversation_id: None,
            initial_snapshot_token: None,
        };
        if let Some(handoff) = self.pending_handoff.as_mut() {
            handoff.submission_state = HandoffSubmissionState::Starting;
            request.conversation_id = Some(handoff.forked_conversation_id.clone());
        }
        self.spawn_agent_with_request(request, ctx);
    }

    #[cfg(not(all(feature = "local_fs", not(target_family = "wasm"))))]
    pub(crate) fn submit_handoff(
        &mut self,
        prompt: String,
        attachments: Vec<AttachmentInput>,
        ctx: &mut ModelContext<Self>,
    ) {
        self.spawn_agent(prompt, attachments, ctx);
    }

    pub fn cancel_task(&mut self, ctx: &mut ModelContext<Self>) {
        self.status = Status::Cancelled {
            progress: AgentProgress::new_stopped(),
        };
        self.pending_followup_prompt = None;
        ctx.emit(AmbientAgentViewModelEvent::Cancelled);
    }

    fn fail_disabled(&mut self, ctx: &mut ModelContext<Self>) {
        self.status = Status::Failed {
            progress: AgentProgress::new_stopped(),
            error_message: OSS_DISABLED_MESSAGE.to_string(),
        };
        self.pending_followup_prompt = None;
        ctx.emit(AmbientAgentViewModelEvent::Failed {
            error_message: OSS_DISABLED_MESSAGE.to_string(),
        });
    }
}

#[derive(Debug, Clone)]
pub enum AmbientAgentViewModelEvent {
    EnteredSetupState,
    EnteredComposingState,
    DispatchedAgent,
    FollowupDispatched,
    ProgressUpdated,
    SessionReady { session_id: SessionId },
    FollowupSessionReady { session_id: SessionId },
    EnvironmentSelected,
    Failed { error_message: String },
    ShowCloudAgentCapacityModal,
    ShowAICreditModal,
    NeedsGithubAuth,
    Cancelled,
    HarnessSelected,
    HostSelected,
    HarnessCommandStarted,
    PendingHandoffChanged,
    HandoffSnapshotUploadFailed { error_message: String },
    UpdatedSetupCommandVisibility,
}

impl Entity for AmbientAgentViewModel {
    type Event = AmbientAgentViewModelEvent;
}
