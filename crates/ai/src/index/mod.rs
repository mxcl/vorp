mod file_outline;
pub mod locations;
pub const DEFAULT_SYNC_REQUESTS_PER_MIN: u32 = 600;

#[cfg(all(feature = "local_fs", not(feature = "oss_release")))]
pub mod full_source_code_embedding;
#[cfg(any(not(feature = "local_fs"), feature = "oss_release"))]
pub mod full_source_code_embedding_unavailable;
#[cfg(any(not(feature = "local_fs"), feature = "oss_release"))]
pub use full_source_code_embedding_unavailable as full_source_code_embedding;

#[cfg(feature = "local_fs")]
pub use file_outline::build_outline;

pub use file_outline::{Outline, Symbol};
#[cfg(feature = "local_fs")]
pub use repo_metadata::{BuildTreeError, DirectoryEntry, Entry, FileId, FileMetadata};

cfg_if::cfg_if! {
    if #[cfg(feature = "local_fs")] {
        pub use repo_metadata::{
            matches_gitignores, path_passes_filters,
        };
    }
}

#[cfg(feature = "local_fs")]
use native::*;

#[cfg(feature = "local_fs")]
mod native {
    use std::thread::available_parallelism;

    pub(super) const MAX_PARALLEL_THREADS: usize = 2;

    fn create_thread_pool() -> Option<rayon::ThreadPool> {
        let num_threads = available_parallelism()
            .map(|parallelism| (parallelism.get() / 2).clamp(1, MAX_PARALLEL_THREADS))
            .unwrap_or(MAX_PARALLEL_THREADS);

        rayon::ThreadPoolBuilder::new()
            .thread_name(|index| format!("warp-code-indexing-{index}"))
            .num_threads(num_threads)
            .build()
            .ok()
    }

    lazy_static::lazy_static! {
        pub(super) static ref THREADPOOL: Option<rayon::ThreadPool> = create_thread_pool();
    }
}
