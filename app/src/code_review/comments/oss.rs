#![allow(dead_code)]

use crate::{
    ai::agent::{CurrentHead, DiffBase},
    code::editor::{line::EditorLineLocation, EditorReviewComment},
    code_review::diff_state::DiffMode,
};
use chrono::{DateTime, Local};
use std::{
    fmt::{Display, Formatter},
    path::{Path, PathBuf},
};
use warp_editor::render::model::LineCount;
use warp_multi_agent_api::{self as api};
use warpui::{Entity, ModelContext};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum CommentOrigin {
    #[default]
    Native,
    ImportedFromGitHub(ImportedCommentDetails),
}

impl CommentOrigin {
    pub(crate) fn is_imported_from_github(&self) -> bool {
        matches!(self, Self::ImportedFromGitHub(_))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportedCommentDetails {
    pub author: String,
    pub github_comment_id: String,
    pub github_parent_id: Option<String>,
    pub html_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct LineDiffContent {
    pub content: String,
    pub lines_added: LineCount,
    pub lines_removed: LineCount,
}

impl LineDiffContent {
    pub(crate) fn original_text(&self) -> String {
        let s = self.content.trim_end_matches('\n');
        s.strip_prefix('+')
            .or_else(|| s.strip_prefix('-'))
            .unwrap_or(s)
            .to_string()
    }

    pub(crate) fn from_content(diff_line: &str) -> Self {
        let lines_added = LineCount::from(if diff_line.starts_with('+') { 1 } else { 0 });
        let lines_removed = LineCount::from(if diff_line.starts_with('-') { 1 } else { 0 });
        Self {
            content: diff_line.to_owned(),
            lines_added,
            lines_removed,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CommentId(uuid::Uuid);

impl CommentId {
    pub(crate) fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub(crate) fn from_uuid(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}

impl Default for CommentId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for CommentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttachedReviewComment {
    pub id: CommentId,
    pub content: String,
    pub target: AttachedReviewCommentTarget,
    pub last_update_time: DateTime<Local>,
    pub base: Option<DiffBase>,
    pub head: Option<CurrentHead>,
    pub outdated: bool,
    pub origin: CommentOrigin,
}

impl From<AttachedReviewComment> for api::ReviewComment {
    fn from(val: AttachedReviewComment) -> Self {
        let comment_target = match val.target {
            AttachedReviewCommentTarget::Line {
                absolute_file_path,
                content,
                line,
            } => {
                let line_range = line.line_number().map(|lc| {
                    let line_number = lc.as_usize() as u32;
                    api::FileContentLineRange {
                        start: line_number,
                        end: line_number + 1,
                    }
                });

                api::review_comment::CommentTarget::CommentedLine(api::DiffHunk {
                    file_path: absolute_file_path.to_string_lossy().to_string(),
                    line_range,
                    diff_content: content.content,
                    lines_added: content.lines_added.as_u32(),
                    lines_removed: content.lines_removed.as_u32(),
                    current: val.head.to_owned().map(Into::into),
                    base: val.base.map(Into::into),
                })
            }
            AttachedReviewCommentTarget::File { absolute_file_path } => {
                api::review_comment::CommentTarget::CommentedFile(
                    api::review_comment::CommentedFile {
                        file_path: absolute_file_path.to_string_lossy().to_string(),
                        current: val.head.to_owned().map(Into::into),
                        base: val.base.map(Into::into),
                    },
                )
            }
            AttachedReviewCommentTarget::General => {
                api::review_comment::CommentTarget::CommentedDiffset(
                    api::review_comment::CommentedDiffset {
                        current: val.head.to_owned().map(Into::into),
                        base: val.base.map(Into::into),
                    },
                )
            }
        };

        api::ReviewComment {
            id: val.id.to_string(),
            comment: val.content,
            comment_target: Some(comment_target),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AttachedReviewCommentTarget {
    Line {
        absolute_file_path: PathBuf,
        line: EditorLineLocation,
        content: LineDiffContent,
    },
    File {
        absolute_file_path: PathBuf,
    },
    General,
}

impl AttachedReviewCommentTarget {
    pub(crate) fn absolute_file_path(&self) -> Option<&PathBuf> {
        match self {
            Self::Line {
                absolute_file_path, ..
            }
            | Self::File { absolute_file_path } => Some(absolute_file_path),
            Self::General => None,
        }
    }

    pub(crate) fn line_number(&self) -> Option<LineCount> {
        match self {
            Self::Line { line, .. } => line.line_number(),
            _ => None,
        }
    }
}

impl AttachedReviewComment {
    pub(crate) fn from_editor_review_comment(
        comment: EditorReviewComment,
        absolute_file_path: PathBuf,
        base: Option<DiffBase>,
        head: Option<CurrentHead>,
    ) -> Self {
        Self {
            id: comment.id,
            content: comment.comment_content,
            target: AttachedReviewCommentTarget::Line {
                absolute_file_path,
                line: comment.line,
                content: comment.diff_content,
            },
            last_update_time: comment.last_update_time,
            base,
            head,
            outdated: false,
            origin: CommentOrigin::Native,
        }
    }

    pub fn head(&self) -> Option<&CurrentHead> {
        self.head.as_ref()
    }

    pub fn origin(&self) -> &CommentOrigin {
        &self.origin
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReviewCommentBatchEvent {
    Changed { should_reposition_comments: bool },
}

#[derive(Clone, Debug, Default)]
pub struct ReviewCommentBatch {
    pub comments: Vec<AttachedReviewComment>,
}

impl Entity for ReviewCommentBatch {
    type Event = ReviewCommentBatchEvent;
}

impl ReviewCommentBatch {
    pub fn from_comments(comments: Vec<AttachedReviewComment>) -> Self {
        Self { comments }
    }

    pub(crate) fn get_review_comment_by_id(&self, id: CommentId) -> Option<&AttachedReviewComment> {
        self.comments.iter().find(|comment| comment.id == id)
    }

    pub(crate) fn diffset_comment(&self) -> Option<&AttachedReviewComment> {
        self.comments
            .iter()
            .find(|comment| matches!(comment.target, AttachedReviewCommentTarget::General))
    }

    pub(crate) fn has_only_outdated_comments(&self) -> bool {
        self.comments.iter().all(|comment| comment.outdated)
    }

    pub fn file_comments<'a>(
        &'a self,
        file: &'a Path,
    ) -> impl Iterator<Item = &'a AttachedReviewComment> + 'a {
        self.comments.iter().filter(move |comment| {
            comment
                .target
                .absolute_file_path()
                .is_some_and(|comment_file| comment_file.ends_with(file))
        })
    }

    pub fn comment_line_numbers_for_file<'a>(
        &'a self,
        file: &'a Path,
    ) -> impl Iterator<Item = LineCount> + 'a {
        self.file_comments(file)
            .filter_map(move |comment| comment.target.line_number())
    }

    pub(crate) fn editor_comments_for_file(&self, file: &Path) -> Vec<EditorReviewComment> {
        self.file_comments(file)
            .filter_map(|comment| EditorReviewComment::try_from(comment.clone()).ok())
            .collect()
    }

    pub(crate) fn upsert_comment(
        &mut self,
        comment: AttachedReviewComment,
        ctx: &mut ModelContext<Self>,
    ) {
        self.upsert_comments_inner(vec![comment]);
        ctx.emit(ReviewCommentBatchEvent::Changed {
            should_reposition_comments: false,
        });
    }

    #[cfg(feature = "local_fs")]
    pub(crate) fn upsert_imported_comments(
        &mut self,
        comments: Vec<AttachedReviewComment>,
        ctx: &mut ModelContext<Self>,
    ) {
        self.upsert_comments_inner(comments);
        ctx.emit(ReviewCommentBatchEvent::Changed {
            should_reposition_comments: true,
        });
    }

    pub fn upsert_comments(
        &mut self,
        comments: Vec<AttachedReviewComment>,
        ctx: &mut ModelContext<Self>,
    ) {
        self.upsert_comments_inner(comments);
        ctx.emit(ReviewCommentBatchEvent::Changed {
            should_reposition_comments: false,
        });
    }

    fn upsert_comments_inner(&mut self, comments: Vec<AttachedReviewComment>) {
        for comment in comments {
            if let Some(existing) = self
                .comments
                .iter_mut()
                .find(|entry| entry.id == comment.id)
            {
                *existing = comment;
            } else {
                self.comments.push(comment);
            }
        }
    }

    pub(crate) fn take_comments(&mut self) -> Vec<AttachedReviewComment> {
        std::mem::take(&mut self.comments)
    }

    pub(crate) fn delete_comment(&mut self, id: CommentId, ctx: &mut ModelContext<Self>) {
        self.comments.retain(|comment| comment.id != id);
        ctx.emit(ReviewCommentBatchEvent::Changed {
            should_reposition_comments: false,
        });
    }

    pub(crate) fn clear_all(&mut self, ctx: &mut ModelContext<Self>) {
        self.comments.clear();
        ctx.emit(ReviewCommentBatchEvent::Changed {
            should_reposition_comments: false,
        });
    }

    #[cfg(feature = "local_fs")]
    pub(crate) fn add_pending_imported_comments(
        &mut self,
        _comments: Vec<PendingImportedReviewComment>,
        _base_branch: DiffMode,
        ctx: &mut ModelContext<Self>,
    ) {
        ctx.emit(ReviewCommentBatchEvent::Changed {
            should_reposition_comments: true,
        });
    }

    pub(crate) fn take_pending_imported_comments_for_branch(
        &mut self,
        _branch: &DiffMode,
    ) -> Vec<PendingImportedReviewComment> {
        Vec::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingImportedReviewComment {
    pub(crate) github_details: ImportedCommentDetails,
    pub(crate) body: String,
    pub(crate) last_update_time: DateTime<Local>,
    pub(crate) target: PendingImportedReviewCommentTarget,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum PendingImportedReviewCommentTarget {
    Line {
        relative_file_path: PathBuf,
        line: EditorLineLocation,
        diff_content: LineDiffContent,
    },
    File {
        relative_file_path: PathBuf,
    },
    General,
}

impl PendingImportedReviewCommentTarget {
    pub(crate) fn file_path(&self) -> Option<&PathBuf> {
        match self {
            Self::Line {
                relative_file_path, ..
            }
            | Self::File { relative_file_path } => Some(relative_file_path),
            Self::General => None,
        }
    }
}

pub(crate) fn convert_insert_review_comments(
    _comments: &[ai::agent::action::InsertReviewComment],
) -> Vec<PendingImportedReviewComment> {
    Vec::new()
}

pub(crate) fn attach_pending_imported_comments(
    _pending_comments: Vec<PendingImportedReviewComment>,
    _repo_path: &Path,
) -> Vec<AttachedReviewComment> {
    Vec::new()
}
