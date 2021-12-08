//! This crate defines the core datastructure representing IDE state -- `RootDatabase`.
//!
//! It is mainly a `HirDatabase` for semantic analysis, plus a `SymbolsDatabase`, for fuzzy search.
#![allow(dead_code, unused)]
mod apply_change;

use common::*;

pub mod assists;
pub mod defs;
pub mod helpers;
pub mod items_locator;
pub mod label;
pub mod path_transform;
pub mod source_change;
pub mod symbol_index;
pub mod ty_filter;

pub mod active_parameter;
pub mod rename;
pub mod search;

use std::{
    fmt,
    mem::ManuallyDrop,
    sync::{Arc, RwLock},
};

use hir::db::{AstDatabase, DefDatabase, HirDatabase};
use rustc_hash::FxHashSet;

use crate::symbol_index::SymbolsDatabase;

/// `vfs` is normally also needed in places where `husky_lang_db` is used, so this re-export is for convenience.
pub use vfs;

pub type FxIndexSet<T> = indexmap::IndexSet<T, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>;
pub type FxIndexMap<K, V> =
    indexmap::IndexMap<K, V, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>;

#[salsa::database(
    vfs::FileQueryGroupStorage,
    // vfs::SourceDatabaseExtStorage,
    // LineIndexDatabaseStorage,
    // symbol_index::SymbolsDatabaseStorage,
    // hir::db::InternDatabaseStorage,
    // hir::db::AstDatabaseStorage,
    // hir::db::DefDatabaseStorage,
    // hir::db::DiagDatabaseStorage,
    // hir::db::HirDatabaseStorage
)]
pub struct HuskyLangDatabase {
    // We use `ManuallyDrop` here because every codegen unit that contains a
    // `&RootDatabase -> &dyn OtherDatabase` cast will instantiate its drop glue in the vtable,
    // which duplicates `Weak::drop` and `Arc::drop` tens of thousands of times, which makes
    // compile times of all `ide_*` and downstream crates suffer greatly.
    storage: ManuallyDrop<salsa::Storage<HuskyLangDatabase>>,
    file_interner: Arc<RwLock<vfs::SourceFileInterner>>,
    package_interner: Arc<RwLock<vfs::PackageInterner>>,
}
impl vfs::VirtualFileSystem for HuskyLangDatabase {
    fn file_interner(&self) -> Arc<RwLock<vfs::SourceFileInterner>> {
        self.file_interner.clone()
    }
    fn package_interner(&self) -> Arc<RwLock<vfs::PackageInterner>> {
        self.package_interner.clone()
    }
}
impl Drop for HuskyLangDatabase {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.storage);
        }
    }
}

impl fmt::Debug for HuskyLangDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RootDatabase").finish()
    }
}

impl salsa::Database for HuskyLangDatabase {}

impl Default for HuskyLangDatabase {
    fn default() -> HuskyLangDatabase {
        HuskyLangDatabase::new(None)
    }
}

impl HuskyLangDatabase {
    pub fn new(lru_capacity: Option<usize>) -> HuskyLangDatabase {
        let mut db = HuskyLangDatabase {
            storage: ManuallyDrop::new(salsa::Storage::default()),
            file_interner: Arc::new(RwLock::new(vfs::SourceFileInterner::default())),
            package_interner: Arc::new(RwLock::new(vfs::PackageInterner::default())),
        };
        // db.set_local_roots_with_durability(Default::default(), Durability::HIGH);
        // db.set_library_roots_with_durability(Default::default(), Durability::HIGH);
        db.update_lru_capacity(lru_capacity);
        db
    }

    pub fn update_lru_capacity(&mut self, lru_capacity: Option<usize>) {
        const DEFAULT_LRU_CAP: usize = 128;

        let lru_capacity = lru_capacity.unwrap_or(DEFAULT_LRU_CAP);
        // todo!();
        // salsa::ParseQuery
        //     .in_db_mut(self)
        //     .set_lru_capacity(lru_capacity);
    }

    pub fn on_diagnostic_change(
        &self,
        f: &dyn Fn(vfs::SourceFileId, Vec<hir::Diagnostic>) -> Result<()>,
    ) -> Result<()> {
        // self.diagnostics();
        todo!()
    }
}

impl salsa::ParallelDatabase for HuskyLangDatabase {
    fn snapshot(&self) -> salsa::Snapshot<HuskyLangDatabase> {
        salsa::Snapshot::new(HuskyLangDatabase {
            storage: ManuallyDrop::new(self.storage.snapshot()),
            file_interner: self.file_interner.clone(),
            package_interner: self.package_interner.clone(),
        })
    }
}

pub struct HuskyLangDatabaseSnapshot {
    snapshot: salsa::Snapshot<HuskyLangDatabase>,
}

#[salsa::query_group(LineIndexDatabaseStorage)]
pub trait LineIndexDatabase: vfs::VirtualFileSystem {
    fn line_map(&self, file_id: vfs::SourceFileId) -> Arc<vfs::LineMap>;
}

fn line_map(db: &dyn LineIndexDatabase, file_id: vfs::SourceFileId) -> Arc<vfs::LineMap> {
    todo!()
    // let text = db.file_text(file_id);
    // Arc::new(LineMap::new(&*text))
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
