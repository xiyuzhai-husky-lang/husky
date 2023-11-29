mod ast;
mod decl;
mod sema_expr;
mod syn_expr;
mod syn_tree;
mod token;

pub(crate) use self::ast::*;
pub(crate) use self::decl::*;
pub(crate) use self::sema_expr::*;
pub(crate) use self::syn_expr::*;
pub(crate) use self::syn_tree::*;
pub(crate) use self::token::*;

use crate::*;

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct DiagnosticSheet {
    pub item_tree_diagnostic_sheet: EntityTreeDiagnosticSheet,
    pub token_diagnostic_sheet: TokenDiagnosticSheet,
    pub ast_diagnostic_sheet: AstDiagnosticSheet,
    pub expr_diagnostic_sheet: ExprDiagnosticSheet,
    pub decl_diagnostic_sheet: DeclDiagnosticSheet,
    pub expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn diagnostic_sheet(db: &::salsa::Db, module_path: ModulePath) -> DiagnosticSheet {
    DiagnosticSheet::new(
        db,
        item_tree_diagnostic_sheet(db, module_path),
        token_diagnostic_sheet(db, module_path),
        ast_diagnostic_sheet(db, module_path),
        expr_diagnostic_sheet(db, module_path),
        decl_diagnostic_sheet(db, module_path),
        expr_ty_diagnostic_sheet(db, module_path),
    )
}

impl DiagnosticSheet {
    pub fn diagnostic_iter<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> impl Iterator<Item = &'a Diagnostic> + 'a {
        self.item_tree_diagnostic_sheet(db)
            .diagnostics(db)
            .iter()
            .chain(self.token_diagnostic_sheet(db).diagnostics(db).iter())
            .chain(self.ast_diagnostic_sheet(db).diagnostics(db).iter())
            .chain(self.expr_diagnostic_sheet(db).diagnostics(db).iter())
            .chain(self.decl_diagnostic_sheet(db).diagnostics(db).iter())
            .chain(self.expr_ty_diagnostic_sheet(db).diagnostics(db).iter())
    }
}

#[test]
fn diagnostic_sheet_works() {
    DB::default().ast_expect_test_debug_with_db(
        DiagnosticsDb::diagnostic_sheet,
        &AstTestConfig::new("diagnostic_sheet"),
    );
}
