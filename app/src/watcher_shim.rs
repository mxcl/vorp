use std::{
    collections::{HashMap, HashSet},
    future::{self, Ready},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use anyhow::Result;
use warpui::{Entity, ModelContext, SingletonEntity};

pub mod notify {
    use super::*;

    #[derive(Clone, Copy)]
    pub enum RecursiveMode {
        Recursive,
        NonRecursive,
    }

    #[derive(Clone)]
    pub struct WatchFilter;

    impl WatchFilter {
        pub fn accept_all() -> Self {
            Self
        }

        pub fn with_filter(_filter: Arc<dyn Fn(&Path) -> bool + Send + Sync>) -> Self {
            Self
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct BulkFilesystemWatcherEvent {
    pub added: HashSet<PathBuf>,
    pub modified: HashSet<PathBuf>,
    pub deleted: HashSet<PathBuf>,
    pub moved: HashMap<PathBuf, PathBuf>,
}

impl BulkFilesystemWatcherEvent {
    pub fn added_or_updated_iter(&self) -> impl Iterator<Item = &PathBuf> {
        self.added.iter().chain(self.modified.iter())
    }

    pub fn added_or_updated_set(&self) -> HashSet<PathBuf> {
        self.added_or_updated_iter().cloned().collect()
    }
}

pub struct BulkFilesystemWatcher;

impl BulkFilesystemWatcher {
    pub fn new(_debounce_duration: Duration, _ctx: &mut ModelContext<Self>) -> Self {
        Self
    }

    pub fn new_for_test() -> Self {
        Self
    }

    pub fn unregister_path(&mut self, _path: &Path) -> Ready<Result<()>> {
        future::ready(Ok(()))
    }

    pub fn register_path(
        &mut self,
        _path: &Path,
        _watch_filter: notify::WatchFilter,
        _recursive_mode: notify::RecursiveMode,
    ) -> Ready<Result<()>> {
        future::ready(Ok(()))
    }
}

impl Entity for BulkFilesystemWatcher {
    type Event = BulkFilesystemWatcherEvent;
}

pub enum HomeDirectoryWatcherEvent {
    HomeFilesChanged(BulkFilesystemWatcherEvent),
}

pub struct HomeDirectoryWatcher;

impl HomeDirectoryWatcher {
    pub fn new(_home_dir: PathBuf, _ctx: &mut ModelContext<Self>) -> Self {
        Self
    }

    pub fn new_for_test(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }
}

impl Entity for HomeDirectoryWatcher {
    type Event = HomeDirectoryWatcherEvent;
}

impl SingletonEntity for HomeDirectoryWatcher {}
