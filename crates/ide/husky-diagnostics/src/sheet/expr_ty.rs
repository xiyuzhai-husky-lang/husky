use husky_expr::{
    EntityPathExpr, EntityPathExprError, Expr, ExprIdx, ExprRegion, OriginalEntityPathExprError,
    OriginalExprError, Stmt, StmtError,
};
use husky_expr_ty::{
    ExprTermError, ExprTypeError, OriginalExprTermError, OriginalExprTypeError,
    OriginalLocalTermExpectationError, OriginalLocalTermResolveError,
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
            collect_expr_ty_diagnostics(db, decl.expr_region(db), &mut diagnostics);
            if let Some(expr_region) = defn.expr_region(db) {
                collect_expr_ty_diagnostics(db, expr_region, &mut diagnostics);
            }
        }
    }
    // todo
    ExprTypeDiagnosticSheet::new(db, diagnostics)
}

fn collect_expr_ty_diagnostics(
    db: &dyn DiagnosticsDb,
    expr_region: ExprRegion,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let ctx: DiagnosticsRegionContext = DiagnosticsRegionContext::new(db, expr_region);
    let expr_ty_region = ctx.expr_ty_region();
    for (expr_idx, local_term_result) in expr_ty_region.expr_local_terms().key_value_iter() {
        match local_term_result {
            Err(ExprTermError::Original(error)) => {
                diagnostics.push((expr_idx, error).to_diagnostic(&ctx))
            }
            _ => (),
        }
    }
    for (expr_idx, ty_info) in expr_ty_region.expr_ty_infos().key_value_iter() {
        match ty_info.ty_result() {
            Err(ExprTypeError::Original(error)) => {
                diagnostics.push((expr_idx, error).to_diagnostic(&ctx))
            }
            _ => (),
        }
    }
    let local_term_table = expr_ty_region.local_term_table();
    for error in local_term_table
        .unresolved_terms()
        .iter()
        .filter_map(|entry| entry.original_error())
    {
        diagnostics.push(error.to_diagnostic(&ctx))
    }
    for (expr_idx, error) in local_term_table
        .expectations()
        .iter()
        .filter_map(|entry| Some((entry.src_expr_idx(), entry.original_error()?)))
    {
        diagnostics.push((expr_idx, error).to_diagnostic(&ctx))
    }
}

impl Diagnose for (ExprIdx, &'_ OriginalExprTermError) {
    type Context<'a> = DiagnosticsRegionContext<'a>;

    fn message(&self, db: &DiagnosticsRegionContext) -> String {
        match self {
            _ => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        todo!()
    }

    fn range(&self, ctx: &DiagnosticsRegionContext) -> TextRange {
        todo!()
    }
}

impl Diagnose for (ExprIdx, &'_ OriginalExprTypeError) {
    type Context<'a> = DiagnosticsRegionContext<'a>;

    fn message(&self, ctx: &DiagnosticsRegionContext) -> String {
        match self {
            _ => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        todo!()
    }

    fn range(&self, ctx: &DiagnosticsRegionContext) -> TextRange {
        todo!()
    }
}

impl Diagnose for OriginalLocalTermResolveError {
    type Context<'a> = DiagnosticsRegionContext<'a>;

    fn message(&self, db: &DiagnosticsRegionContext) -> String {
        match self {
            OriginalLocalTermResolveError::UnresolvedTerm => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        todo!()
    }

    fn range(&self, ctx: &DiagnosticsRegionContext) -> TextRange {
        todo!()
    }
}

impl Diagnose for (ExprIdx, &'_ OriginalLocalTermExpectationError) {
    type Context<'a> = DiagnosticsRegionContext<'a>;

    fn message(&self, db: &DiagnosticsRegionContext) -> String {
        match self.1 {
            OriginalLocalTermExpectationError::Type(_) => {
                format!("OriginalLocalTermExpectationError type error: todo")
            }
            OriginalLocalTermExpectationError::Todo => {
                format!("OriginalLocalTermExpectationError::Todo: todo")
            }
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &DiagnosticsRegionContext) -> TextRange {
        ctx.expr_range(self.0)
    }
}
