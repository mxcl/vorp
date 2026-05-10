use warpui::{AppContext, Entity, EntityId};

use crate::ai::execution_profiles::profiles::ClientProfileId;
use crate::search::data_source::{Query, QueryResult};
use crate::search::mixer::DataSourceRunErrorWrapper;
use crate::search::SyncDataSource;
use crate::terminal::input::inline_menu::{InlineMenuAction, InlineMenuType};

#[derive(Clone, Debug)]
pub enum SelectProfileMenuItem {
    Profile { profile_id: ClientProfileId },
    ManageProfiles,
}

impl InlineMenuAction for SelectProfileMenuItem {
    const MENU_TYPE: InlineMenuType = InlineMenuType::ProfileSelector;
}

pub struct ProfileSelectorDataSource {
    _terminal_view_id: EntityId,
}

impl ProfileSelectorDataSource {
    pub fn new(terminal_view_id: EntityId) -> Self {
        Self {
            _terminal_view_id: terminal_view_id,
        }
    }
}

impl SyncDataSource for ProfileSelectorDataSource {
    type Action = SelectProfileMenuItem;

    fn run_query(
        &self,
        _query: &Query,
        _app: &AppContext,
    ) -> Result<Vec<QueryResult<Self::Action>>, DataSourceRunErrorWrapper> {
        Ok(Vec::new())
    }
}

impl Entity for ProfileSelectorDataSource {
    type Event = ();
}
