//! This crate defines the core datastructure representing IDE state -- `RootDatabase`.
//!
//! It is mainly a `HirDatabase` for semantic analysis, plus a `SymbolsDatabase`, for fuzzy search.
#![allow(dead_code, unused)]
mod apply_change;

pub mod assists;
pub mod defs;
pub mod helpers;
pub mod items_locator;
pub mod label;
pub mod line_index;
pub mod path_transform;
pub mod source_change;
pub mod symbol_index;
pub mod ty_filter;

pub mod active_parameter;
pub mod rename;
pub mod search;

use std::{fmt, mem::ManuallyDrop, sync::Arc};

use file_db::{
    salsa::{self, Durability},
    AnchoredPath, CrateId, FileDatabase, FileID, FileLoader, FileLoaderDelegate,
};
use hir::db::{AstDatabase, DefDatabase, HirDatabase};
use rustc_hash::FxHashSet;

use crate::{line_index::LineIndex, symbol_index::SymbolsDatabase};

/// `file_db` is normally also needed in places where `ide_db` is used, so this re-export is for convenience.
pub use file_db;

pub type FxIndexSet<T> = indexmap::IndexSet<T, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>;
pub type FxIndexMap<K, V> =
    indexmap::IndexMap<K, V, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>;

#[salsa::database(
    file_db::FileDatabaseStorage,
    // file_db::SourceDatabaseExtStorage,
    // LineIndexDatabaseStorage,
    // symbol_index::SymbolsDatabaseStorage,
    // hir::db::InternDatabaseStorage,
    // hir::db::AstDatabaseStorage,
    // hir::db::DefDatabaseStorage,
    // hir::db::DiagDatabaseStorage,
    // hir::db::HirDatabaseStorage
)]
pub struct IdeDatabase {
    // We use `ManuallyDrop` here because every codegen unit that contains a
    // `&RootDatabase -> &dyn OtherDatabase` cast will instantiate its drop glue in the vtable,
    // which duplicates `Weak::drop` and `Arc::drop` tens of thousands of times, which makes
    // compile times of all `ide_*` and downstream crates suffer greatly.
    storage: ManuallyDrop<salsa::Storage<IdeDatabase>>,
}

impl Drop for IdeDatabase {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.storage);
        }
    }
}

impl fmt::Debug for IdeDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RootDatabase").finish()
    }
}

// impl Upcast<dyn AstDatabase> for IdeDatabase {
//     fn upcast(&self) -> &(dyn AstDatabase + 'static) {
//         todo!()
//     }
// }

// impl Upcast<dyn DefDatabase> for IdeDatabase {
//     fn upcast(&self) -> &(dyn DefDatabase + 'static) {
//         todo!()
//     }
// }

// impl Upcast<dyn HirDatabase> for IdeDatabase {
//     fn upcast(&self) -> &(dyn HirDatabase + 'static) {
//         todo!()
//     }
// }

impl FileLoader for IdeDatabase {
    fn file_text(&self, file_id: FileID) -> Arc<String> {
        todo!()
        // FileLoaderDelegate(self).file_text(file_id)
    }
    fn resolve_path(&self, path: AnchoredPath) -> Option<FileID> {
        todo!()
        // FileLoaderDelegate(self).resolve_path(path)
    }
    fn relevant_crates(&self, file_id: FileID) -> Arc<FxHashSet<CrateId>> {
        todo!()
        // FileLoaderDelegate(self).relevant_crates(file_id)
    }
}

impl salsa::Database for IdeDatabase {}

impl Default for IdeDatabase {
    fn default() -> IdeDatabase {
        IdeDatabase::new(None)
    }
}

impl IdeDatabase {
    pub fn new(lru_capacity: Option<usize>) -> IdeDatabase {
        todo!()
        // let mut db = IdeDatabase {
        //     storage: ManuallyDrop::new(salsa::Storage::default()),
        // };
        // db.set_local_roots_with_durability(Default::default(), Durability::HIGH);
        // db.set_library_roots_with_durability(Default::default(), Durability::HIGH);
        // db.update_lru_capacity(lru_capacity);
        // db
    }

    pub fn update_lru_capacity(&mut self, lru_capacity: Option<usize>) {
        todo!()
        // let lru_capacity = lru_capacity.unwrap_or(file_db::DEFAULT_LRU_CAP);
        // file_db::ParseQuery
        //     .in_db_mut(self)
        //     .set_lru_capacity(lru_capacity);
    }
}

impl salsa::ParallelDatabase for IdeDatabase {
    fn snapshot(&self) -> salsa::Snapshot<IdeDatabase> {
        salsa::Snapshot::new(IdeDatabase {
            storage: ManuallyDrop::new(self.storage.snapshot()),
        })
    }
}

#[salsa::query_group(LineIndexDatabaseStorage)]
pub trait LineIndexDatabase: file_db::FileDatabase {
    fn line_index(&self, file_id: FileID) -> Arc<LineIndex>;
}

fn line_index(db: &dyn LineIndexDatabase, file_id: FileID) -> Arc<LineIndex> {
    let text = db.file_text(file_id);
    Arc::new(LineIndex::new(&*text))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SymbolKind {
    Const,
    ConstParam,
    Enum,
    Field,
    Function,
    Impl,
    Label,
    Local,
    Macro,
    Module,
    SelfParam,
    Static,
    Struct,
    Trait,
    TypeAlias,
    TypeParam,
    Union,
    ValueParam,
    Variant,
}
