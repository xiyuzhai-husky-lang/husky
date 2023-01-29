mod ast;
mod entity_tree;
mod expr;
mod token;

pub use ast::*;
pub use entity_tree::*;
pub use expr::*;
pub use token::*;

use crate::*;

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct DiagnosticSheet {
    pub token_diagnostic_sheet: TokenDiagnosticSheet,
    pub ast_diagnostic_sheet: AstDiagnosticSheet,
    pub expr_diagnostic_sheet: ExprDiagnosticSheet,
    pub entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn diagnostic_sheet(db: &dyn DiagnosticsDb, module_path: ModulePath) -> DiagnosticSheet {
    let token_diagnostic_sheet = token_diagnostic_sheet(db, module_path);
    let ast_diagnostic_sheet = ast_diagnostic_sheet(db, module_path);
    let expr_diagnostic_sheet = expr_diagnostic_sheet(db, module_path);
    DiagnosticSheet::new(
        db,
        token_diagnostic_sheet,
        ast_diagnostic_sheet,
        expr_diagnostic_sheet,
        entity_tree_diagnostic_sheet(db, module_path),
    )
}

impl DiagnosticSheet {
    pub fn diagnostic_iter<'a>(
        self,
        db: &'a dyn DiagnosticsDb,
    ) -> impl Iterator<Item = &'a Diagnostic> + 'a {
        self.token_diagnostic_sheet(db)
            .diagnostics(db)
            .iter()
            .chain(self.ast_diagnostic_sheet(db).diagnostics(db).iter())
            .chain(self.expr_diagnostic_sheet(db).diagnostics(db).iter())
            .chain(self.entity_tree_diagnostic_sheet(db).diagnostics(db).iter())
    }
}

#[test]
fn diagnostic_sheet_works() {
    DB::default().vfs_expect_test_debug_with_db("diagnostic_sheet", DiagnosticsDb::diagnostic_sheet)
}
