#![doc = include_str!("../README.md")]
#![deny(unsafe_code, missing_docs, clippy::unwrap_used)]

use std::sync::Arc;

/// alias for `Arc<Vec<Arc<T>>>`
pub type Avec<T> = Arc<Vec<Arc<T>>>;
