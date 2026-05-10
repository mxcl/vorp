use warpui::{AppContext, Entity, EntityId};

use crate::ai::llms::LLMId;
use crate::search::data_source::{Query, QueryResult};
use crate::search::mixer::DataSourceRunErrorWrapper;
use crate::search::SyncDataSource;
use crate::terminal::input::inline_menu::{InlineMenuAction, InlineMenuType};

#[derive(Clone, Debug)]
pub struct AcceptModel {
    pub id: LLMId,
}

impl InlineMenuAction for AcceptModel {
    const MENU_TYPE: InlineMenuType = InlineMenuType::ModelSelector;
}

pub struct ModelSelectorDataSource {
    _terminal_view_id: EntityId,
}

impl ModelSelectorDataSource {
    pub fn new(terminal_view_id: EntityId) -> Self {
        Self {
            _terminal_view_id: terminal_view_id,
        }
    }
}

impl SyncDataSource for ModelSelectorDataSource {
    type Action = AcceptModel;

    fn run_query(
        &self,
        _query: &Query,
        _app: &AppContext,
    ) -> Result<Vec<QueryResult<Self::Action>>, DataSourceRunErrorWrapper> {
        Ok(Vec::new())
    }
}

impl Entity for ModelSelectorDataSource {
    type Event = ();
}
