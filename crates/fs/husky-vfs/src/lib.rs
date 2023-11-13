#![feature(iter_advance_by)]
#![allow(incomplete_features)]
#![feature(absolute_path)]
#![feature(trait_upcasting)]
#![feature(let_chains)]
mod cache;
pub mod db;
pub mod error;
mod file;
#[cfg(feature = "lsp_support")]
mod lsp_support;
mod path;
pub mod snippet;
#[cfg(feature = "test-utils")]
pub mod test_utils;
#[cfg(test)]
mod tests;
mod toolchain;
mod watch;

pub use self::cache::VfsCache;
pub use self::file::Notebook;
#[cfg(feature = "lsp_support")]
pub use self::lsp_support::*;
pub use self::path::*;
#[cfg(feature = "test-utils")]
pub use self::test_utils::*;
pub use self::toolchain::*;
pub use self::watch::{VfsWatcher, WatchableVfsDb, WatchedVfs};

use self::error::*;
use self::file::*;
use dashmap::{mapref::entry::Entry, DashMap};
use husky_coword::*;
use notify_debouncer_mini::notify::RecursiveMode;

use salsa::storage::HasJar;
use std::path::{Path, PathBuf};
#[cfg(test)]
use tests::*;
