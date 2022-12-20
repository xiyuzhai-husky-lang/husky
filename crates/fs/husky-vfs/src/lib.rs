#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(let_chains)]
mod cache;
mod db;
mod error;
mod file;
mod jar;
mod path;
mod runner;
#[cfg(feature = "test_utils")]
mod test_utils;
#[cfg(test)]
mod tests;
mod watch;

pub use cache::VfsCache;
pub use db::VfsDb;
pub use error::*;
pub use jar::VfsJar;
pub use runner::*;
#[cfg(feature = "test_utils")]
pub use test_utils::*;
pub use watch::{VfsWatcher, WatchableVfsDb, WatchedVfs};

use dashmap::{mapref::entry::Entry, DashMap};

use file::*;
use husky_print_utils::p;
use notify_debouncer_mini::notify::RecursiveMode;
use path::*;
use salsa::storage::HasJar;
use std::path::{Path, PathBuf};
#[cfg(test)]
use tests::*;

#[salsa::interned(jar = VfsJar)]
pub struct PathBufItd {
    #[return_ref]
    path: PathBuf,
}
