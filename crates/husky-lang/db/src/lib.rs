use common::*;

use std::{fmt, mem::ManuallyDrop, sync::Arc};

use stdx::sync::ARwLock;

use file;

pub type FxIndexSet<T> = indexmap::IndexSet<T, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>;
pub type FxIndexMap<K, V> =
    indexmap::IndexMap<K, V, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>;

#[salsa::database(
    file::FileQueryGroupStorage,
    // lex::LexQueryGroupStorage,
    // syntax::SyntaxQueryGroupStorage,
    // semantic::SemanticQueryGroupStorage,
    // diagnostic::DiagnosticQueryGroupStorage,
    // LineIndexDatabaseStorage,
    // symbol_index::SymbolsDatabaseStorage,
    // hir::db::InternDatabaseStorage,
    // hir::db::AstDatabaseStorage,
    // hir::db::DefDatabaseStorage,
    // hir::db::DiagDatabaseStorage,
    // hir::db::HirDatabaseStorage
)]
#[derive(Default)]
pub struct HuskyLangDatabase {
    // We use `ManuallyDrop` here because every codegen unit that contains a
    // `&RootDatabase -> &dyn OtherDatabase` cast will instantiate its drop glue in the vtable,
    // which duplicates `Weak::drop` and `Arc::drop` tens of thousands of times, which makes
    // compile times of all `ide_*` and downstream crates suffer greatly.
    storage: ManuallyDrop<salsa::Storage<HuskyLangDatabase>>,
    source_file_interner: file::FileInterner,
    live_docs: ARwLock<HashMap<file::FileId, Arc<String>>>,
}
impl Drop for HuskyLangDatabase {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.storage);
        }
    }
}
impl file::InternFile for HuskyLangDatabase {
    fn provide_source_interner(&self) -> &file::FileInterner {
        &self.source_file_interner
    }
}
impl file::LiveFiles for HuskyLangDatabase {
    fn get_live_docs(&self) -> &ARwLock<HashMap<file::FileId, Arc<String>>> {
        &self.live_docs
    }

    fn did_change_source(&mut self, id: file::FileId) {
        file::FileContentQuery.in_db_mut(self).invalidate(&id);
    }
}

impl fmt::Debug for HuskyLangDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RootDatabase").finish()
    }
}

impl salsa::Database for HuskyLangDatabase {}

impl HuskyLangDatabase {
    pub fn new(lru_capacity: Option<usize>) -> HuskyLangDatabase {
        let mut db = HuskyLangDatabase::default();
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

    pub fn on_diagnostic_change<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce(file::FileId, Vec<diagnostic::Diagnostic>) -> Result<()>,
    {
        // self.diagnostics();
        todo!()
    }
}

impl salsa::ParallelDatabase for HuskyLangDatabase {
    fn snapshot(&self) -> salsa::Snapshot<HuskyLangDatabase> {
        salsa::Snapshot::new(HuskyLangDatabase {
            storage: ManuallyDrop::new(self.storage.snapshot()),
            source_file_interner: self.source_file_interner.clone(),
            live_docs: self.live_docs.clone(),
        })
    }
}

pub struct HuskyLangDatabaseSnapshot {
    snapshot: salsa::Snapshot<HuskyLangDatabase>,
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
