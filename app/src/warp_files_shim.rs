use std::{
    collections::{HashMap, HashSet},
    io,
    ops::Range,
    path::{Path, PathBuf},
    rc::Rc,
    time::SystemTime,
};

use warp_core::HostId;
use warp_util::{
    content_version::ContentVersion,
    file::{FileId, FileLoadError, FileSaveError},
    standardized_path::StandardizedPath,
};
use warpui::{Entity, ModelContext, SingletonEntity};

#[derive(Debug)]
pub enum FileModelEvent {
    FileLoaded {
        content: String,
        id: FileId,
        version: ContentVersion,
    },
    FailedToLoad {
        id: FileId,
        error: Rc<FileLoadError>,
    },
    FileSaved {
        id: FileId,
        version: ContentVersion,
    },
    FailedToSave {
        id: FileId,
        error: Rc<FileSaveError>,
    },
    FileUpdated {
        id: FileId,
        content: String,
        base_version: ContentVersion,
        new_version: ContentVersion,
    },
}

impl FileModelEvent {
    pub fn file_id(&self) -> FileId {
        match self {
            Self::FileLoaded { id, .. }
            | Self::FailedToLoad { id, .. }
            | Self::FileSaved { id, .. }
            | Self::FailedToSave { id, .. }
            | Self::FileUpdated { id, .. } => *id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextFileSegment {
    pub file_name: String,
    pub content: String,
    pub line_range: Option<Range<usize>>,
    pub last_modified: Option<SystemTime>,
    pub line_count: usize,
}

pub enum TextFileReadResult {
    Segments {
        segments: Vec<TextFileSegment>,
        bytes_read: usize,
    },
    NotText,
}

enum FileBackend {
    Local(PathBuf),
    Remote {
        host_id: HostId,
        path: StandardizedPath,
    },
}

pub struct FileModel {
    files: HashMap<FileId, FileBackend>,
    versions: HashMap<FileId, ContentVersion>,
}

impl FileModel {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self {
            files: HashMap::new(),
            versions: HashMap::new(),
        }
    }

    pub fn file_path(&self, file_id: FileId) -> Option<PathBuf> {
        match self.files.get(&file_id) {
            Some(FileBackend::Local(path)) => Some(path.clone()),
            Some(FileBackend::Remote { .. }) | None => None,
        }
    }

    pub fn register_remote_file(&mut self, host_id: HostId, path: StandardizedPath) -> FileId {
        let file_id = FileId::new();
        self.files
            .insert(file_id, FileBackend::Remote { host_id, path });
        file_id
    }

    pub fn register_file_path(
        &mut self,
        file_path: &Path,
        _subscribe_to_updates: bool,
        _ctx: &mut ModelContext<Self>,
    ) -> FileId {
        let file_id = FileId::new();
        self.files
            .insert(file_id, FileBackend::Local(file_path.to_owned()));
        file_id
    }

    pub fn open(
        &mut self,
        file_path: &Path,
        _subscribe_to_updates: bool,
        ctx: &mut ModelContext<Self>,
    ) -> FileId {
        let file_id = FileId::new();
        let file_path = file_path.to_owned();
        self.files
            .insert(file_id, FileBackend::Local(file_path.clone()));

        ctx.spawn(
            async move {
                let contents = async_fs::read_to_string(&file_path)
                    .await
                    .map_err(FileLoadError::from);
                (file_id, contents)
            },
            move |me, (file_id, load_result), ctx| match load_result {
                Ok(content) => {
                    let version = ContentVersion::new();
                    me.set_version(file_id, version);
                    ctx.emit(FileModelEvent::FileLoaded {
                        content,
                        id: file_id,
                        version,
                    });
                }
                Err(error) => ctx.emit(FileModelEvent::FailedToLoad {
                    id: file_id,
                    error: Rc::new(error),
                }),
            },
        );

        file_id
    }

    pub async fn read_content_for_file(file_path: &Path) -> Result<String, FileLoadError> {
        if !Self::file_exists(file_path).await {
            return Err(FileLoadError::DoesNotExist);
        }
        async_fs::read_to_string(file_path)
            .await
            .map_err(FileLoadError::from)
    }

    pub async fn read_lines_async(
        file_path: &Path,
        line_numbers: Vec<usize>,
    ) -> Result<Vec<(usize, String)>, FileLoadError> {
        if line_numbers.is_empty() {
            return Ok(Vec::new());
        }
        let content = Self::read_content_for_file(file_path).await?;
        let requested: HashSet<usize> = line_numbers.into_iter().collect();
        Ok(content
            .lines()
            .enumerate()
            .filter_map(|(line_number, line)| {
                requested
                    .contains(&line_number)
                    .then(|| (line_number, line.to_owned()))
            })
            .collect())
    }

    pub async fn read_text_file(
        path: &Path,
        max_bytes: usize,
        requested_ranges: &[Range<usize>],
        last_modified: Option<SystemTime>,
    ) -> anyhow::Result<TextFileReadResult> {
        let bytes = async_fs::read(path).await?;
        let content = match String::from_utf8(bytes) {
            Ok(content) => content,
            Err(_) => return Ok(TextFileReadResult::NotText),
        };

        let line_count = content.lines().count();
        let file_name = path.to_string_lossy().to_string();
        let ranges = if requested_ranges.is_empty() {
            std::iter::once(1..usize::MAX).collect()
        } else {
            requested_ranges.to_vec()
        };

        let mut bytes_read = 0;
        let mut segments = Vec::new();
        for range in ranges {
            if bytes_read >= max_bytes {
                break;
            }

            let mut segment_content = String::new();
            for (line_index, line) in content.lines().enumerate() {
                let line_number = line_index + 1;
                if line_number < range.start || line_number >= range.end {
                    continue;
                }

                if !segment_content.is_empty() {
                    segment_content.push('\n');
                }
                segment_content.push_str(line);
                if bytes_read + segment_content.len() >= max_bytes {
                    segment_content.truncate(max_bytes.saturating_sub(bytes_read));
                    break;
                }
            }

            bytes_read += segment_content.len();
            segments.push(TextFileSegment {
                file_name: file_name.clone(),
                content: segment_content,
                line_range: requested_ranges
                    .is_empty()
                    .then_some(None)
                    .unwrap_or_else(|| Some(range)),
                last_modified,
                line_count,
            });
        }

        Ok(TextFileReadResult::Segments {
            segments,
            bytes_read,
        })
    }

    pub async fn read_file_as_binary(file_path: &Path) -> Result<Vec<u8>, FileLoadError> {
        if !Self::file_exists(file_path).await {
            return Err(FileLoadError::DoesNotExist);
        }
        async_fs::read(file_path).await.map_err(FileLoadError::from)
    }

    pub async fn file_exists(file_path: &Path) -> bool {
        async_fs::metadata(file_path).await.is_ok()
    }

    pub async fn create_file(file_path: &Path) -> Result<(), io::Error> {
        async_fs::File::create(file_path).await.map(|_| ())
    }

    pub async fn ensure_parent_directories(path: &Path) -> Result<(), io::Error> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                async_fs::create_dir_all(parent).await?;
            }
        }
        Ok(())
    }

    pub fn cancel(&mut self, _file_id: FileId) {}

    pub fn unsubscribe(&mut self, file_id: FileId, _ctx: &mut ModelContext<Self>) {
        self.files.remove(&file_id);
        self.versions.remove(&file_id);
    }

    pub fn save(
        &mut self,
        file_id: FileId,
        content: String,
        version: ContentVersion,
        ctx: &mut ModelContext<Self>,
    ) -> Result<(), FileSaveError> {
        let backend = self
            .files
            .get(&file_id)
            .ok_or(FileSaveError::NoFilePath(file_id))?;

        match backend {
            FileBackend::Local(file_path) => {
                let file_path = file_path.clone();
                ctx.spawn(
                    async move {
                        Self::ensure_parent_directories(&file_path)
                            .await
                            .map_err(|error| FileSaveError::IOError {
                                error,
                                path: file_path.clone(),
                            })?;
                        async_fs::write(&file_path, content).await.map_err(|error| {
                            FileSaveError::IOError {
                                error,
                                path: file_path,
                            }
                        })
                    },
                    move |me, result, ctx| me.emit_save_result(file_id, version, result, ctx),
                );
            }
            FileBackend::Remote { host_id, path } => {
                let error = FileSaveError::RemoteError(format!(
                    "Remote file support is not available in this build: {host_id}:{}",
                    path.as_str()
                ));
                ctx.emit(FileModelEvent::FailedToSave {
                    id: file_id,
                    error: Rc::new(error),
                });
            }
        }

        Ok(())
    }

    pub fn rename_and_save(
        &mut self,
        file_id: FileId,
        new_path: PathBuf,
        content: String,
        version: ContentVersion,
        ctx: &mut ModelContext<Self>,
    ) -> Result<(), FileSaveError> {
        let file_path = self
            .file_path(file_id)
            .ok_or(FileSaveError::NoFilePath(file_id))?;
        self.files
            .insert(file_id, FileBackend::Local(new_path.clone()));

        ctx.spawn(
            async move {
                Self::ensure_parent_directories(&new_path)
                    .await
                    .map_err(|error| FileSaveError::IOError {
                        error,
                        path: new_path.clone(),
                    })?;
                async_fs::write(&file_path, content)
                    .await
                    .map_err(|error| FileSaveError::IOError {
                        error,
                        path: file_path.clone(),
                    })?;
                async_fs::rename(&file_path, &new_path)
                    .await
                    .map_err(|error| FileSaveError::IOError {
                        error,
                        path: file_path,
                    })
            },
            move |me, result, ctx| me.emit_save_result(file_id, version, result, ctx),
        );

        Ok(())
    }

    pub fn delete(
        &mut self,
        file_id: FileId,
        version: ContentVersion,
        ctx: &mut ModelContext<Self>,
    ) -> Result<(), FileSaveError> {
        let backend = self
            .files
            .get(&file_id)
            .ok_or(FileSaveError::NoFilePath(file_id))?;

        match backend {
            FileBackend::Local(file_path) => {
                let file_path = file_path.clone();
                ctx.spawn(
                    async move {
                        async_fs::remove_file(&file_path).await.map_err(|error| {
                            FileSaveError::IOError {
                                error,
                                path: file_path,
                            }
                        })
                    },
                    move |me, result, ctx| me.emit_save_result(file_id, version, result, ctx),
                );
            }
            FileBackend::Remote { host_id, path } => {
                let error = FileSaveError::RemoteError(format!(
                    "Remote file support is not available in this build: {host_id}:{}",
                    path.as_str()
                ));
                ctx.emit(FileModelEvent::FailedToSave {
                    id: file_id,
                    error: Rc::new(error),
                });
            }
        }

        Ok(())
    }

    pub fn set_version(&mut self, file_id: FileId, version: ContentVersion) {
        self.versions.insert(file_id, version);
    }

    pub fn version(&self, file_id: FileId) -> Option<ContentVersion> {
        self.versions.get(&file_id).copied()
    }

    fn emit_save_result(
        &mut self,
        file_id: FileId,
        version: ContentVersion,
        result: Result<(), FileSaveError>,
        ctx: &mut ModelContext<Self>,
    ) {
        match result {
            Ok(()) => {
                self.set_version(file_id, version);
                ctx.emit(FileModelEvent::FileSaved {
                    id: file_id,
                    version,
                });
            }
            Err(error) => ctx.emit(FileModelEvent::FailedToSave {
                id: file_id,
                error: Rc::new(error),
            }),
        }
    }
}

impl Entity for FileModel {
    type Event = FileModelEvent;
}

impl SingletonEntity for FileModel {}
