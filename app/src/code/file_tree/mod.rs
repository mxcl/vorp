//! File picker component for rendering expandable folder structures.

pub mod snapshot;

#[cfg(not(feature = "oss_release"))]
#[cfg_attr(not(feature = "local_fs"), allow(dead_code, unused_imports))]
mod view;
#[cfg(feature = "oss_release")]
#[path = "view_oss.rs"]
mod view;

pub use view::*;
