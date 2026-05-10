#[cfg(not(feature = "oss_release"))]
mod batch;
#[cfg(not(feature = "oss_release"))]
mod comment;
#[cfg(not(feature = "oss_release"))]
pub(crate) mod convert;
#[cfg(not(feature = "oss_release"))]
mod diff_hunk_parser;
#[cfg(not(feature = "oss_release"))]
mod flatten;
#[cfg(feature = "oss_release")]
mod oss;
#[cfg(not(feature = "oss_release"))]
mod pending_imported;

#[cfg(not(feature = "oss_release"))]
pub(crate) use batch::{ReviewCommentBatch, ReviewCommentBatchEvent};
#[cfg(not(feature = "oss_release"))]
pub(crate) use comment::{
    AttachedReviewComment, AttachedReviewCommentTarget, CommentId, CommentOrigin, LineDiffContent,
};
#[cfg(not(feature = "oss_release"))]
pub(crate) use convert::convert_insert_review_comments;
#[cfg(not(feature = "oss_release"))]
pub(crate) use flatten::attach_pending_imported_comments;
#[cfg(feature = "oss_release")]
pub(crate) use oss::*;
#[cfg(not(feature = "oss_release"))]
pub(crate) use pending_imported::{
    PendingImportedReviewComment, PendingImportedReviewCommentTarget,
};
