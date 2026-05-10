#[cfg(not(feature = "oss_release"))]
mod data_source;
#[cfg(feature = "oss_release")]
#[path = "data_source_oss.rs"]
mod data_source;
#[cfg(not(feature = "oss_release"))]
mod model_spec_scores;
#[cfg(not(feature = "oss_release"))]
mod view;
#[cfg(feature = "oss_release")]
#[path = "view_oss.rs"]
mod view;

pub use data_source::{AcceptModel, ModelSelectorDataSource};
pub use view::{InlineModelSelectorEvent, InlineModelSelectorTab, InlineModelSelectorView};
