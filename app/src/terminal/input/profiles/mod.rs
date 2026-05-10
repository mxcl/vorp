#[cfg(not(feature = "oss_release"))]
mod data_source;
#[cfg(feature = "oss_release")]
#[path = "data_source_oss.rs"]
mod data_source;
#[cfg(not(feature = "oss_release"))]
mod search_item;
#[cfg(not(feature = "oss_release"))]
mod view;
#[cfg(feature = "oss_release")]
#[path = "view_oss.rs"]
mod view;

pub use data_source::SelectProfileMenuItem;
pub use view::{InlineProfileSelectorEvent, InlineProfileSelectorView};
