#[cfg(test)]
mod tests;

pub use file::{FileQuery, InternFile, LiveFiles};
pub use scope::{InternScope, ScopeQuery};
pub use word::InternWord;

use common::*;

use std::fmt;

use stdx::sync::ARwLock;

pub type FxIndexSet<T> = indexmap::IndexSet<T, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>;
pub type FxIndexMap<K, V> =
    indexmap::IndexMap<K, V, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>;

#[salsa::database(
    file::FileQueryStorage,
    token::TokenQueryStorage,
    scope::ScopeQueryStorage,
    diagnostic::DiagnosticQueryStorage,
    atom::AtomQueryStorage
)]
pub struct HuskyLangDatabase {
    storage: salsa::Storage<HuskyLangDatabase>,
    file_interner: file::FileInterner,
    word_interner: word::WordInterner,
    scope_interner: scope::ScopeInterner,
    live_docs: ARwLock<HashMap<file::FileId, ARwLock<String>>>,
}
impl Default for HuskyLangDatabase {
    fn default() -> Self {
        Self {
            storage: Default::default(),
            file_interner: file::new_file_interner(),
            word_interner: word::new_word_interner(),
            scope_interner: scope::new_scope_interner(),
            live_docs: Default::default(),
        }
    }
}
impl InternFile for HuskyLangDatabase {
    fn provide_file_interner(&self) -> &file::FileInterner {
        &self.file_interner
    }
}
impl InternWord for HuskyLangDatabase {
    fn word_interner(&self) -> &word::WordInterner {
        &self.word_interner
    }
}
impl LiveFiles for HuskyLangDatabase {
    fn get_live_files(&self) -> &ARwLock<HashMap<file::FileId, ARwLock<String>>> {
        &self.live_docs
    }

    fn did_change_source(&mut self, id: file::FileId) {
        file::FileContentQuery.in_db_mut(self).invalidate(&id);
    }
}

impl FileQuery for HuskyLangDatabase {}

impl InternScope for HuskyLangDatabase {
    fn provide_scope_interner(&self) -> &scope::ScopeInterner {
        &self.scope_interner
    }
}
impl ScopeQuery for HuskyLangDatabase {}

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
        if lru_capacity.is_some() {
            todo!()
            // const _DEFAULT_LRU_CAP: usize = 128;
            // msg_once!("TODO: update_lru_capacity");
            // let lru_capacity = lru_capacity.unwrap_or(DEFAULT_LRU_CAP);
            // // todo!();
            // file::FileQuery
            //     .in_db_mut(self)
            //     .set_lru_capacity(lru_capacity);
        }
    }
}

impl salsa::ParallelDatabase for HuskyLangDatabase {
    fn snapshot(&self) -> salsa::Snapshot<HuskyLangDatabase> {
        salsa::Snapshot::new(HuskyLangDatabase {
            storage: self.storage.snapshot(),
            file_interner: self.file_interner.clone(),
            word_interner: self.word_interner.clone(),
            scope_interner: self.scope_interner.clone(),
            live_docs: self.live_docs.clone(),
        })
    }
}
