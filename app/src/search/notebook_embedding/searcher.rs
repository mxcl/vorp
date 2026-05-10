use crate::search::mixer::SearchMixer;
use crate::server::ids::SyncId;

pub type EmbeddingSearchMixer = SearchMixer<EmbeddingSearchItemAction>;

#[derive(Clone, Debug)]
pub enum EmbeddingSearchItemAction {
    AcceptWorkflow(SyncId),
    #[cfg(not(feature = "oss_release"))]
    AcceptNotebook(SyncId),
}
