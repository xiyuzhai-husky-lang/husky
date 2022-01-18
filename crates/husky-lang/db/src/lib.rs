#[cfg(test)]
mod tests;

pub use ast::AstQueryGroup;
pub use diagnostic::DiagnosticQuery;
pub use file::{FileQuery, InternFile, LiveFiles};
pub use husky_fmt::FmtQuery;
pub use scope::{InternScope, Scope};
pub use scope_query::{ScopeQueryGroup, ScopeSalsaQueryGroup};
pub use semantics::PackageQueryGroup;
pub use word::InternWord;

use common::*;

use std::fmt;

use stdx::sync::ARwLock;

#[salsa::database(
    file::FileQueryStorage,
    token::TokenQueryGroupStorage,
    scope_query::ScopeQueryGroupStorage,
    ast::AstQueryGroupStorage,
    husky_fmt::FormatQueryGroupStorage,
    semantics::PackageQueryGroupStorage,
    semantics::MainQueryGroupStorage,
    semantics::EntityQueryGroupStorage,
    semantics::ConfigQueryGroupStorage,
    semantics::CallSignatureQueryGroupStorage,
    diagnostic::DiagnosticQueryStorage
)]
pub struct HuskyLangDatabase {
    storage: salsa::Storage<HuskyLangDatabase>,
    file_interner: file::FileInterner,
    word_interner: word::WordInterner,
    scope_interner: scope::ScopeInterner,
    live_docs: ARwLock<HashMap<file::FileId, ARwLock<String>>>,
}

impl fmt::Debug for HuskyLangDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HuskyLangDatabase").finish()
    }
}

impl salsa::Database for HuskyLangDatabase {}

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

impl HuskyLangDatabase {
    pub fn new() -> HuskyLangDatabase {
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
    fn file_interner(&self) -> &file::FileInterner {
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
    fn scope_interner(&self) -> &scope::ScopeInterner {
        &self.scope_interner
    }
}

impl ScopeQueryGroup for HuskyLangDatabase {}

impl semantics::LazyStmtQueryGroup for HuskyLangDatabase {
    fn as_lazy_stmt_query_group(&self) -> &dyn semantics::LazyStmtQueryGroup {
        self
    }
}
