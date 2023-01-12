mod ast;
mod entity_tree;
mod expr;
mod token;

pub use ast::*;
pub use entity_tree::*;
pub use expr::*;
pub use token::*;

use crate::*;

#[salsa::tracked(jar = DiagnosticsJar)]
pub struct DiagnosticSheet {
    pub ast_diagnostic_sheet: AstDiagnosticSheet,
    pub entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet,
    pub token_diagnostic_sheet: TokenDiagnosticSheet,
}

// ad hoc
impl<Db: DiagnosticsDb> salsa::DebugWithDb<Db> for DiagnosticSheet {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn DiagnosticsDb, include_all_fields)
    }
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn diagnostic_sheet(db: &dyn DiagnosticsDb, module_path: ModulePath) -> DiagnosticSheet {
    DiagnosticSheet::new(
        db,
        ast_diagnostic_sheet(db, module_path),
        entity_tree_diagnostic_sheet(db, module_path),
        token_diagnostic_sheet(db, module_path),
    )
}

impl DiagnosticSheet {
    pub fn diagnostic_iter<'a>(
        self,
        db: &'a dyn DiagnosticsDb,
    ) -> impl Iterator<Item = &'a Diagnostic> + 'a {
        self.ast_diagnostic_sheet(db)
            .diagnostics(db)
            .iter()
            .chain(self.entity_tree_diagnostic_sheet(db).diagnostics(db).iter())
            .chain(self.token_diagnostic_sheet(db).diagnostics(db).iter())
    }
}

#[test]
fn diagnostic_sheet_works() {
    DB::expect_test_probable_modules_debug_with_db(
        "diagnostic_sheet",
        DiagnosticsDb::diagnostic_sheet,
    )
}
