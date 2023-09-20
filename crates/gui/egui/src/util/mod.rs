//! Miscellaneous tools used by the rest of egui.

pub mod cache;
pub(crate) mod fixed_cache;
pub mod id_type_map;
pub mod undoer;

pub use id_type_map::IdTypeMap;

pub use husky_epaint::emath::History;
pub use husky_epaint::util::{hash, hash_with};
