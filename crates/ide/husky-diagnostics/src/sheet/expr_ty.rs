use super::*;
use husky_expr::{ExprIdx, ExprRegion};
use husky_expr_ty::{ExprTermError, ExprTypeError, OriginalExprTermError, OriginalExprTypeError};
use husky_fluffy_term::*;
use salsa::{DebugWithDb, DisplayWithDb};

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
        db.collect_defns(module_path),
    ) {
        let _token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        for (_, defn) in defn_sheet.defns() {
            if let Ok(defn) = defn {
                let decl = defn.decl(db);
                collect_expr_ty_diagnostics(db, decl.expr_region(db), &mut diagnostics);
                if let Some(expr_region) = defn.expr_region(db) {
                    collect_expr_ty_diagnostics(db, expr_region, &mut diagnostics);
                }
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
    for (expr_idx, fluffy_term_result) in expr_ty_region.expr_fluffy_terms().key_value_iter() {
        match fluffy_term_result {
            Err(ExprTermError::Original(error)) => {
                diagnostics.push((expr_idx, error).to_diagnostic(&ctx))
            }
            _ => (),
        }
    }
    for (expr_idx, ty_info) in expr_ty_region.expr_ty_infos().key_value_iter() {
        match ty_info.ty() {
            Err(ExprTypeError::Original(error)) => {
                diagnostics.push((expr_idx, error).to_diagnostic(&ctx))
            }
            _ => (),
        }
    }
    for (expr_idx, error) in expr_ty_region.extra_expr_ty_errors() {
        match error {
            ExprTypeError::Original(error) => {
                diagnostics.push((*expr_idx, error).to_diagnostic(&ctx))
            }
            _ => (),
        }
    }
    let fluffy_term_region = expr_ty_region.fluffy_term_region();
    for (src, error) in fluffy_term_region.hollow_terms().errors() {
        diagnostics.push((src, error).to_diagnostic(&ctx))
    }
    for (src, error) in fluffy_term_region
        .expectations()
        .iter()
        .filter_map(|entry| Some((entry.src(), entry.original_error()?)))
    {
        diagnostics.push((src, error).to_diagnostic(&ctx))
    }
}

impl Diagnose for (ExprIdx, &'_ OriginalExprTermError) {
    type Context<'a> = RegionDiagnosticsContext<'a>;

    fn message(&self, _db: &RegionDiagnosticsContext) -> String {
        match self {
            _ => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        todo!()
    }

    fn range(&self, _ctx: &RegionDiagnosticsContext) -> TextRange {
        todo!()
    }
}

impl Diagnose for (ExprIdx, &'_ OriginalExprTypeError) {
    type Context<'a> = RegionDiagnosticsContext<'a>;

    fn message(&self, ctx: &RegionDiagnosticsContext) -> String {
        // MOM
        match self.1 {
            OriginalExprTypeError::UnresolvedTerm => {
                format!("Type Error: UnresolvedTerm")
            }
            OriginalExprTypeError::TypeMethodTypeError => format!("TypeError: "),
            OriginalExprTypeError::TypeCallTypeError => format!("TypeError: "),
            OriginalExprTypeError::TodoScopeResolution => {
                format!("Type Error: TodoScopeResolution")
            }
            OriginalExprTypeError::TodoSuffix => {
                format!("Type Error: TodoSuffix")
            }
            OriginalExprTypeError::TodoBoxColon => {
                format!("Type Error: TodoBoxColon")
            }
            OriginalExprTypeError::FinalDestination => {
                format!("Type Error: final destination")
            }
            OriginalExprTypeError::FugitivePathTypeError => {
                format!("Type Error: form path error")
            }
            OriginalExprTypeError::AmbiguousTypePath => {
                format!("Type Error: AmbiguousTypePath")
            }
            OriginalExprTypeError::RitchieCallWrongNumberOfArguments {
                number_of_nonself_parameters,
                number_of_nonself_arguments,
            } => {
                format!("expected {number_of_nonself_parameters} argument, found {number_of_nonself_arguments}")
            }
            OriginalExprTypeError::AmbiguousListExpr => {
                format!("Type Error: AmbiguateListExpr")
            }
            OriginalExprTypeError::AmbiguousTildeExpr => {
                format!("Type Error: AmbiguateTildeExpr")
            }
            OriginalExprTypeError::NoSuchField => {
                format!("Type Error: NoSuchField")
            }
            OriginalExprTypeError::NoMethodForType {
                self_expr_ty_unravelled,
                ident_token,
            } => {
                todo!()
                // format!(
                //     "Type Error: no method named `{}` for type `{:?}`",
                //     ident_token.ident().data(ctx.db()),
                //     self_expr_ty_unravelled.debug(ctx.db()) // ad hoc
                // )
            }
            OriginalExprTypeError::TodoIndexOrComposeWithList => {
                format!("Type Error: TodoIndexOrComposeWithList")
            }
            OriginalExprTypeError::TodoMemo => {
                format!("Type Error: TodoMemo")
            }
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &RegionDiagnosticsContext) -> TextRange {
        match self.1 {
            OriginalExprTypeError::NoMethodForType { ident_token, .. } => ctx
                .ranged_token_sheet()
                .token_text_range(ident_token.token_idx()),
            _ => ctx.expr_text_range(self.0),
        }
    }
}

impl Diagnose for (HoleSource, &'_ OriginalHollowTermResolveError) {
    type Context<'a> = RegionDiagnosticsContext<'a>;

    fn message(&self, _db: &RegionDiagnosticsContext) -> String {
        match self.1 {
            OriginalHollowTermResolveError::UnresolvedTerm => "unresolved term".to_string(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &RegionDiagnosticsContext) -> TextRange {
        match self.0 {
            HoleSource::Expr(_) => todo!(),
            HoleSource::Expectation(_) => todo!(),
        }
        // ctx.expr_text_range()
    }
}

impl Diagnose for (ExpectationSource, &'_ OriginalFluffyTermExpectationError) {
    type Context<'a> = RegionDiagnosticsContext<'a>;

    fn message(&self, ctx: &RegionDiagnosticsContext) -> String {
        match self.1 {
            OriginalFluffyTermExpectationError::Todo => {
                format!("OriginalFluffyTermExpectationError::Todo")
            }
            OriginalFluffyTermExpectationError::TypePathMismatch {
                expected_path,
                expectee_path,
            } => format!(
                "type path mismatch: expect {}, but got {} instead",
                expected_path.display(ctx.db()),
                expectee_path.display(ctx.db())
            ),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &RegionDiagnosticsContext) -> TextRange {
        ctx.expr_text_range(self.0.expr_idx())
    }
}
