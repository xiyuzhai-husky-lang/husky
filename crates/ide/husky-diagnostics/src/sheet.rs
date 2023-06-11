mod ast;
mod decl;
mod defn;
mod entity_tree;
mod expr;
mod expr_ty;
mod token;

pub(crate) use ast::*;
pub(crate) use decl::*;
pub(crate) use defn::*;
pub(crate) use entity_tree::*;
pub(crate) use expr::*;
pub(crate) use expr_ty::*;
pub(crate) use token::*;

use crate::*;

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct DiagnosticSheet {
    pub entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet,
    pub token_diagnostic_sheet: TokenDiagnosticSheet,
    pub ast_diagnostic_sheet: AstDiagnosticSheet,
    pub expr_diagnostic_sheet: ExprDiagnosticSheet,
    pub decl_diagnostic_sheet: DeclDiagnosticSheet,
    pub defn_diagnostic_sheet: DefnDiagnosticSheet,
    pub expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn diagnostic_sheet(db: &dyn DiagnosticsDb, module_path: ModulePath) -> DiagnosticSheet {
    DiagnosticSheet::new(
        db,
        entity_tree_diagnostic_sheet(db, module_path),
        token_diagnostic_sheet(db, module_path),
        ast_diagnostic_sheet(db, module_path),
        expr_diagnostic_sheet(db, module_path),
        decl_diagnostic_sheet(db, module_path),
        defn_diagnostic_sheet(db, module_path),
        expr_ty_diagnostic_sheet(db, module_path),
    )
}

impl DiagnosticSheet {
    pub fn diagnostic_iter<'a>(
        self,
        db: &'a dyn DiagnosticsDb,
    ) -> impl Iterator<Item = &'a Diagnostic> + 'a {
        todo!();
        self.entity_tree_diagnostic_sheet(db)
            .diagnostics(db)
            .iter()
            .chain(self.token_diagnostic_sheet(db).diagnostics(db).iter())
            .chain(self.ast_diagnostic_sheet(db).diagnostics(db).iter())
            .chain(self.expr_diagnostic_sheet(db).diagnostics(db).iter())
            .chain(self.decl_diagnostic_sheet(db).diagnostics(db).iter())
            .chain(self.defn_diagnostic_sheet(db).diagnostics(db).iter())
            .chain(self.expr_ty_diagnostic_sheet(db).diagnostics(db).iter())
    }
}

#[test]
fn diagnostic_sheet_works() {
    DB::default().ast_expect_test_debug_with_db("diagnostic_sheet", DiagnosticsDb::diagnostic_sheet)
}
