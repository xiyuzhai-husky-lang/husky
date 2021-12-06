use std::sync::Arc;

use super::Diagnostic;

#[salsa::query_group(AstDatabaseStorage)]
pub trait AstDatabase: base_db::SourceDatabase {}

#[salsa::query_group(InternDatabaseStorage)]
pub trait InternDatabase: base_db::SourceDatabase {}

#[salsa::query_group(DefDatabaseStorage)]
pub trait DefDatabase: InternDatabase + AstDatabase {}

#[salsa::query_group(DiagDatabaseStorage)]
pub trait DiagDatabase: DefDatabase {
    #[salsa::invoke(compute_diagnostics)]
    fn diagnostics(&self, file_id: vfs::FileID) -> Vec<Diagnostic>;
}

#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase: DefDatabase + DiagDatabase {}

fn compute_diagnostics(db: &dyn DiagDatabase, file_id: vfs::FileID) -> Vec<Diagnostic> {
    vec![Diagnostic::todo()]
}
