use ai::index::Outline;

cfg_if::cfg_if! {
    if #[cfg(all(not(target_family = "wasm"), feature = "ai_local_fs"))] {
        mod native;
        pub use native::*;
    } else {
        mod wasm;
        pub use wasm::*;
    }
}

#[cfg_attr(target_family = "wasm", allow(dead_code))]
#[derive(Debug)]
pub enum OutlineStatus {
    /// The outline is being computed.
    Pending,
    /// The successfully computed outline.
    Complete(Outline),
    /// Outline creation failed.
    Failed,
}
