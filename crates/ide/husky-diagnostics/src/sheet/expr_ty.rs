use husky_expr::{
    EntityPathExpr, EntityPathExprError, Expr, ExprIdx, ExprRegion, OriginalEntityPathExprError,
    OriginalExprError, Stmt, StmtError,
};
use husky_expr_ty::{
    ExprTermError, ExprTypeError, OriginalExprTermError, OriginalExprTypeError,
    OriginalLocalTermExpectationError, OriginalLocalTermResolveError,
};
use husky_token::RangedTokenSheet;
use salsa::{DebugWithDb, DisplayWithDb};

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
    let ctx: RegionDiagnosticsContext = RegionDiagnosticsContext::new(db, expr_region);
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
    for (expr_idx, error) in local_term_table
        .unresolved_terms()
        .iter()
        .filter_map(|entry| Some((entry.src_expr_idx(), entry.original_error()?)))
    {
        diagnostics.push((expr_idx, error).to_diagnostic(&ctx))
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
    type Context<'a> = RegionDiagnosticsContext<'a>;

    fn message(&self, db: &RegionDiagnosticsContext) -> String {
        match self {
            _ => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        todo!()
    }

    fn range(&self, ctx: &RegionDiagnosticsContext) -> TextRange {
        todo!()
    }
}

impl Diagnose for (ExprIdx, &'_ OriginalExprTypeError) {
    type Context<'a> = RegionDiagnosticsContext<'a>;

    fn message(&self, ctx: &RegionDiagnosticsContext) -> String {
        match self.1 {
            OriginalExprTypeError::UnresolvedTerm => {
                format!("Type Error: UnresolvedTerm")
            }
            OriginalExprTypeError::FieldTypeError(error) => format!("TypeError: {error}"),
            OriginalExprTypeError::TypeMethodTypeError(error) => format!("TypeError: {error}"),
            OriginalExprTypeError::TypeCallTypeError(error) => format!("TypeError: {error}"),
            OriginalExprTypeError::TodoScopeResolution => {
                format!("Type Error: TodoScopeResolution")
            }
            OriginalExprTypeError::TodoSuffix => {
                format!("Type Error: TodoSuffix")
            }
            OriginalExprTypeError::TodoBoxColon => {
                format!("Type Error: TodoBoxColon")
            }
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &RegionDiagnosticsContext) -> TextRange {
        ctx.expr_text_range(self.0)
    }
}

impl Diagnose for (ExprIdx, &'_ OriginalLocalTermResolveError) {
    type Context<'a> = RegionDiagnosticsContext<'a>;

    fn message(&self, db: &RegionDiagnosticsContext) -> String {
        match self.1 {
            OriginalLocalTermResolveError::UnresolvedTerm => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &RegionDiagnosticsContext) -> TextRange {
        ctx.expr_text_range(self.0)
    }
}

impl Diagnose for (ExprIdx, &'_ OriginalLocalTermExpectationError) {
    type Context<'a> = RegionDiagnosticsContext<'a>;

    fn message(&self, ctx: &RegionDiagnosticsContext) -> String {
        match self.1 {
            OriginalLocalTermExpectationError::TermTypeError { term, error } => {
                format!("Type Error: {error} in term {}", term.display(ctx.db()))
            }
            OriginalLocalTermExpectationError::Todo => {
                format!("OriginalLocalTermExpectationError::Todo: todo")
            }
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &RegionDiagnosticsContext) -> TextRange {
        ctx.expr_text_range(self.0)
    }
}
