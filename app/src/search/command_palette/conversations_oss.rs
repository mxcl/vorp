//! OSS no-op command-palette conversation data source.

use crate::ai::agent::conversation::AIConversationId;
use crate::search::command_palette::mixer::CommandPaletteItemAction;
use crate::search::data_source::{Query, QueryResult};
use crate::search::mixer::DataSourceRunErrorWrapper;
use crate::search::SyncDataSource;
use warpui::{AppContext, Entity};

#[derive(Default)]
pub struct DataSource;

impl DataSource {
    pub fn new() -> Self {
        Self
    }

    pub fn historical() -> Self {
        Self
    }

    pub fn query_result(
        _conversation_id: &AIConversationId,
        _app: &AppContext,
    ) -> Option<QueryResult<CommandPaletteItemAction>> {
        None
    }

    pub fn top_n(
        &self,
        _limit: usize,
        _app: &AppContext,
    ) -> impl Iterator<Item = QueryResult<<Self as SyncDataSource>::Action>> {
        std::iter::empty()
    }
}

impl SyncDataSource for DataSource {
    type Action = CommandPaletteItemAction;

    fn run_query(
        &self,
        _query: &Query,
        _app: &AppContext,
    ) -> Result<Vec<QueryResult<Self::Action>>, DataSourceRunErrorWrapper> {
        Ok(Vec::new())
    }
}

impl Entity for DataSource {
    type Event = ();
}
