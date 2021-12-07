//! file_db defines basic database traits. The concrete DB is defined by ide.
#![allow(dead_code, unused)]
mod change;
mod input;

use std::{panic, sync::Arc};

use common::{TextRange, TextSize};
use rustc_hash::FxHashSet;
use syntax::{ParseResult, SingleFileParseTree};

pub use crate::input::{
    CrateDisplayName, CrateId, CrateName, Dependency, Edition, Env, SourceRoot, SourceRootId,
};
pub use salsa::{self, Cancelled};
pub use vfs::{
    file_id_path_table::FilePathIdTable, AnchoredPath, AnchoredPathBuf, FileID, VfsPath,
};

// pub trait Upcast<T: ?Sized> {
//     fn upcast(&self) -> &T;
// }

#[derive(Clone, Copy, Debug)]
pub struct FilePosition {
    pub file_id: FileID,
    pub offset: TextSize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FileRange {
    pub file_id: FileID,
    pub range: TextRange,
}

pub const DEFAULT_LRU_CAP: usize = 128;

pub trait FileLoader {
    /// Text of the file.
    fn file_text(&self, file_id: FileID) -> Arc<String>;
    fn resolve_path(&self, path: AnchoredPath) -> Option<FileID>;
    fn relevant_crates(&self, file_id: FileID) -> Arc<FxHashSet<CrateId>>;
}

/// Database which stores all significant input facts: source code and project
/// model. Everything else in husky-lang-server is derived from these queries.
#[salsa::query_group(FileDatabaseStorage)]
pub trait FileDatabase: FileLoader + std::fmt::Debug {}

#[salsa::query_group(SourceDatabaseExtStorage)]
pub trait SourceDatabaseExt: FileDatabase {
    #[salsa::input]
    fn file_text(&self, file_id: FileID) -> Arc<String>;
    /// Path to a file, relative to the root of its source root.
    /// Source root of the file.
    #[salsa::input]
    fn package_root(&self, file_id: FileID) -> SourceRootId;

    fn source_root_crates(&self, id: SourceRootId) -> Arc<FxHashSet<CrateId>>;
}

fn source_root_crates(db: &dyn SourceDatabaseExt, id: SourceRootId) -> Arc<FxHashSet<CrateId>> {
    todo!()
}

/// Silly workaround for cyclic deps between the traits
pub struct FileLoaderDelegate<T>(pub T);

impl<T: SourceDatabaseExt> FileLoader for FileLoaderDelegate<&'_ T> {
    fn file_text(&self, file_id: FileID) -> Arc<String> {
        SourceDatabaseExt::file_text(self.0, file_id)
    }
    fn resolve_path(&self, path: AnchoredPath) -> Option<FileID> {
        todo!()
    }

    fn relevant_crates(&self, file_id: FileID) -> Arc<FxHashSet<CrateId>> {
        let _p = profile::span("relevant_crates");
        let source_root = self.0.package_root(file_id);
        self.0.source_root_crates(source_root)
    }
}
