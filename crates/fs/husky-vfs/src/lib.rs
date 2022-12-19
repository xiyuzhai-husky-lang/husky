#![feature(trait_upcasting)]
#![feature(let_chains)]
mod cache;
mod db;
mod error;
mod file;
mod jar;
mod path;
mod runner;
#[cfg(test)]
mod tests;
mod watch;

pub use cache::VfsCache;
pub use db::VfsDb;
pub use error::*;
pub use jar::VfsJar;
pub use runner::*;
pub use watch::{VfsWatcher, WatchableVfsDb, WatchedVfs};

use dashmap::{mapref::entry::Entry, DashMap};
use db::*;
use eyre::Context;
use file::*;
use husky_print_utils::p;
use husky_toolchain::*;
use notify_debouncer_mini::{
    notify::{RecommendedWatcher, RecursiveMode},
    Debouncer,
};
use path::*;
use salsa::{storage::HasJar, ParallelDatabase};
use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};

#[salsa::interned(jar = VfsJar)]
pub struct PathBufItd {
    #[return_ref]
    path: PathBuf,
}
