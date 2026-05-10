#[cfg(not(feature = "oss_release"))]
pub mod shared_objects_creation_denied_body;
#[cfg(not(feature = "oss_release"))]
pub mod shared_objects_creation_denied_modal;
#[cfg(feature = "oss_release")]
pub mod shared_objects_creation_denied_modal_oss;
#[cfg(feature = "oss_release")]
pub use shared_objects_creation_denied_modal_oss as shared_objects_creation_denied_modal;
