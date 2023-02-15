use husky_expr::{
    EntityPathExpr, EntityPathExprError, Expr, ExprError, ExprRegion, OriginalEntityPathExprError,
    Stmt, StmtError,
};
use husky_expr_ty::{
    ExprTermError, ExprTypeError, OriginalExprTermError, OriginalLocalTermExpectationError,
    OriginalLocalTermResolveError,
};
use husky_token::RangedTokenSheet;
use salsa::DebugWithDb;

use super::*;

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct ExprTypeDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn expr_ty_diagnostic_sheet(
    db: &dyn DiagnosticsDb,
    module_path: ModulePath,
) -> ExprTypeDiagnosticSheet {
    let mut diagnostics = vec![];
    if let (Ok(ranged_token_sheet), Ok(defn_sheet)) = (
        db.ranged_token_sheet(module_path),
        db.defn_sheet(module_path),
    ) {
        let token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        for defn in defn_sheet.defns() {
            let decl = defn.decl(db);
            collect_expr_ty_diagnostics(
                db,
                ranged_token_sheet,
                token_sheet_data,
                decl.expr_region(db),
                &mut diagnostics,
            );
            if let Some(expr_region) = defn.expr_region(db) {
                collect_expr_ty_diagnostics(
                    db,
                    ranged_token_sheet,
                    token_sheet_data,
                    expr_region,
                    &mut diagnostics,
                );
            }
        }
    }
    // todo
    ExprTypeDiagnosticSheet::new(db, diagnostics)
}

fn collect_expr_ty_diagnostics(
    db: &dyn DiagnosticsDb,
    ranged_token_sheet: &RangedTokenSheet,
    token_sheet_data: &TokenSheetData,
    expr_region: ExprRegion,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let expr_ty_region = db.expr_ty_region(expr_region);
    for local_term_result in expr_ty_region.expr_local_terms().value_iter() {
        match local_term_result {
            Err(ExprTermError::Original(error)) => {
                diagnostics.push(error.to_diagnostic(db, ranged_token_sheet, token_sheet_data))
            }
            _ => (),
        }
    }
    for ty_info in expr_ty_region.expr_ty_infos().value_iter() {
        match ty_info.ty_result() {
            Err(ExprTypeError::Original(error)) => todo!(),
            _ => (),
        }
    }
    let local_term_table = expr_ty_region.local_term_table();
    for error in local_term_table
        .unresolved_terms()
        .iter()
        .filter_map(|entry| entry.original_error())
    {
        diagnostics.push(error.to_diagnostic(db, ranged_token_sheet, token_sheet_data))
    }
    for error in local_term_table
        .expectations()
        .iter()
        .filter_map(|entry| entry.original_error())
    {
        diagnostics.push(error.to_diagnostic(db, ranged_token_sheet, token_sheet_data))
    }
}

impl Diagnose for OriginalExprTermError {
    fn message(&self, db: &dyn DiagnosticsDb) -> String {
        match self {
            _ => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        todo!()
    }

    fn range(
        &self,
        ranged_token_sheet: &RangedTokenSheet,
        token_sheet_data: &TokenSheetData,
    ) -> TextRange {
        todo!()
    }
}

impl Diagnose for OriginalLocalTermResolveError {
    fn message(&self, db: &dyn DiagnosticsDb) -> String {
        match self {
            OriginalLocalTermResolveError::UnresolvedTerm => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        todo!()
    }

    fn range(
        &self,
        ranged_token_sheet: &RangedTokenSheet,
        token_sheet_data: &TokenSheetData,
    ) -> TextRange {
        todo!()
    }
}

impl Diagnose for OriginalLocalTermExpectationError {
    fn message(&self, db: &dyn DiagnosticsDb) -> String {
        match self {
            OriginalLocalTermExpectationError::Type(_) => todo!(),
            OriginalLocalTermExpectationError::Todo => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        todo!()
    }

    fn range(
        &self,
        ranged_token_sheet: &RangedTokenSheet,
        token_sheet_data: &TokenSheetData,
    ) -> TextRange {
        todo!()
    }
}
