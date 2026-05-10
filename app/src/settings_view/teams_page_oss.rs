use super::{
    settings_page::{MatchData, SettingsPageMeta, SettingsPageViewHandle},
    SettingsSection,
};
use crate::{
    auth::UserUid,
    server::ids::ServerId,
    view_components::ToastFlavor,
    workspaces::team::MembershipRole,
};
use serde::{Deserialize, Serialize};
use warpui::{
    elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext, ViewHandle,
};

#[derive(Debug, Clone)]
pub enum TeamsPageAction {
    LeaveTeam,
    ShowLeaveTeamConfirmationDialog,
    ShowDeleteTeamConfirmationDialog,
    CopyLink(String),
    CreateTeam,
    ChangeInviteViewOption(TeamsInviteOption),
    DeletePendingEmailInvitation {
        team_uid: ServerId,
        invitee_email: String,
    },
    RemoveUserFromTeam {
        user_uid: UserUid,
        team_uid: ServerId,
    },
    ToggleIsInviteLinkEnabled {
        team_uid: ServerId,
        current_state: bool,
    },
    ResetInviteLinks {
        team_uid: ServerId,
    },
    AddDomainRestrictions {
        team_uid: ServerId,
    },
    DeleteDomainRestriction {
        domain_uid: ServerId,
        team_uid: ServerId,
    },
    SendEmailInvites {
        team_uid: ServerId,
    },
    OpenWarpDrive,
    GenerateUpgradeLink {
        team_uid: ServerId,
    },
    GenerateStripeBillingPortalLink {
        team_uid: ServerId,
    },
    OpenAdminPanel {
        team_uid: ServerId,
    },
    ContactSupport,
    ToggleTeamDiscoverabilityBeforeCreation,
    ToggleTeamDiscoverability {
        team_uid: ServerId,
        current_state: bool,
    },
    JoinTeamWithTeamDiscovery {
        team_uid: ServerId,
    },
    ShowTransferOwnershipModal {
        new_owner_email: String,
        new_owner_uid: UserUid,
        team_uid: ServerId,
    },
    OpenMemberActionsMenu {
        index: usize,
    },
    CloseMemberActionsMenu,
    SetTeamMemberRole {
        team_uid: ServerId,
        user_uid: UserUid,
        role: MembershipRole,
    },
}

impl TeamsPageAction {
    pub fn blocked_for_anonymous_user(&self) -> bool {
        false
    }
}

#[derive(Clone)]
pub enum TeamsPageViewEvent {
    TeamsChanged,
    OpenWarpDrive,
    ShowToast {
        message: String,
        flavor: ToastFlavor,
    },
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Copy, Serialize, Deserialize)]
pub enum TeamsInviteOption {
    #[default]
    Link,
    Email,
}

impl std::fmt::Display for TeamsInviteOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Link => write!(f, "Link"),
            Self::Email => write!(f, "Email"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OpenTeamsSettingsModalArgs {
    pub invite_email: Option<String>,
}

pub struct TeamsPageView;

impl TeamsPageView {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn open_team_members(&mut self, _email: Option<&String>, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for TeamsPageView {
    type Event = TeamsPageViewEvent;
}

impl TypedActionView for TeamsPageView {
    type Action = TeamsPageAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for TeamsPageView {
    fn ui_name() -> &'static str {
        "TeamsPageView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl SettingsPageMeta for TeamsPageView {
    fn section() -> SettingsSection {
        SettingsSection::Teams
    }

    fn should_render(&self, _ctx: &AppContext) -> bool {
        false
    }

    fn update_filter(&mut self, _query: &str, _ctx: &mut ViewContext<Self>) -> MatchData {
        MatchData::Uncounted(false)
    }

    fn scroll_to_widget(&mut self, _widget_id: &'static str) {}

    fn clear_highlighted_widget(&mut self) {}
}

impl From<ViewHandle<TeamsPageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<TeamsPageView>) -> Self {
        SettingsPageViewHandle::Teams(view_handle)
    }
}
