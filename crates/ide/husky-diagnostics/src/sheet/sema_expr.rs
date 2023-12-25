use super::*;
use husky_fluffy_term::*;
use husky_sema_expr::{
    OriginalExprTermError, OriginalSemaExprDataError, OriginalSemaExprTypeError, SemaExprTermError,
};
use husky_syn_decl::HasSynNodeDecl;
use husky_syn_defn::module_item_syn_node_defns;
use husky_syn_expr::SynExprRegion;
use salsa::DisplayWithDb;

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct ExprTypeDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn expr_ty_diagnostic_sheet(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> ExprTypeDiagnosticSheet {
    let mut diagnostics = vec![];
    for (syn_node_path, defn) in module_item_syn_node_defns(db, module_path) {
        let decl = syn_node_path.syn_node_decl(db);
        if let Some(syn_expr_region) = decl.syn_expr_region(db) {
            collect_expr_ty_diagnostics(db, syn_expr_region, &mut diagnostics);
        }
        if let Some(defn) = defn {
            collect_expr_ty_diagnostics(db, defn.syn_expr_region, &mut diagnostics);
        }
    }
    ExprTypeDiagnosticSheet::new(db, diagnostics)
}

fn collect_expr_ty_diagnostics(
    db: &::salsa::Db,
    syn_expr_region: SynExprRegion,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let ctx: RegionDiagnosticsContext = RegionDiagnosticsContext::new(db, syn_expr_region);
    let sema_expr_region_data = ctx.sema_expr_region_data();
    for (_expr_idx, fluffy_term_result) in sema_expr_region_data.sema_expr_terms().iter() {
        match fluffy_term_result {
            Err(SemaExprTermError::Original(error)) => diagnostics.push(error.to_diagnostic(&ctx)),
            _ => (),
        }
    }
    for sema_expr_entry in sema_expr_region_data.sema_expr_arena().iter() {
        if let Some(e) = sema_expr_entry.original_data_error() {
            diagnostics.push(e.to_diagnostic(&ctx))
        }
        if let Some(e) = sema_expr_entry.original_data_error() {
            diagnostics.push(e.to_diagnostic(&ctx))
        }
    }
    let fluffy_term_region = sema_expr_region_data.fluffy_term_region();
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

impl Diagnose for OriginalExprTermError {
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

impl Diagnose for OriginalSemaExprDataError {
    type Context<'a> = RegionDiagnosticsContext<'a>;

    fn message(&self, ctx: &RegionDiagnosticsContext) -> String {
        // MOM
        match self {
            OriginalSemaExprDataError::NoSuchField {
                owner_ty,
                ident_token,
            } => {
                format!(
                    "Type Error: no field named {} in {}",
                    ident_token.ident().data(ctx.db()),
                    owner_ty.show(ctx.db(), ctx.fluffy_term_region().terms())
                )
            }
            OriginalSemaExprDataError::NoSuchMethod {
                self_expr_ty,
                ident_token,
            } => {
                format!(
                    "Type Error: no method named `{}` for type `{}`",
                    ident_token.ident().data(ctx.db()),
                    self_expr_ty.show(ctx.db(), ctx.fluffy_term_region().terms()) // ad hoc
                )
            }
            OriginalSemaExprDataError::ExpectedIndices => {
                format!("Type Error: expected indices")
            }
            OriginalSemaExprDataError::CannotIndexIntoType { self_expr_ty } => {
                format!(
                    "Type Error: cannot index into type `{}`",
                    self_expr_ty.show(ctx.db(), ctx.fluffy_term_region().terms())
                )
            }
            OriginalSemaExprDataError::RitchieParameterArgumentMismatch {
                match_error: _,
                ritchie_arguments: _,
            } => todo!(), // OriginalSemaExprDataError::UnexpectedArgument => {
                          //     format!("Type Error: unexpected argument")
                          // }
                          // OriginalSemaExprDataError::MissingArgument => {
                          //     format!("Type Error: missing argument")
                          // }
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, _ctx: &RegionDiagnosticsContext) -> TextRange {
        todo!()
        // match self {
        //     OriginalSemaExprTypeError::NoSuchField { ident_token, .. } => {
        //         ctx.token_text_range(ident_token.regional_token_idx())
        //     }
        //     OriginalSemaExprTypeError::NoMethodForType { ident_token, .. } => {
        //         ctx.token_text_range(ident_token.regional_token_idx())
        //     }
        //     _ => ctx.expr_text_range(self.0),
        // }
    }
}

impl Diagnose for OriginalSemaExprTypeError {
    type Context<'a> = RegionDiagnosticsContext<'a>;

    fn message(&self, _ctx: &RegionDiagnosticsContext) -> String {
        // MOM
        match self {
            OriginalSemaExprTypeError::UnresolvedTerm => {
                format!("Type Error: UnresolvedTerm")
            }
            OriginalSemaExprTypeError::TypeMethodTypeError => format!("TypeError: "),
            OriginalSemaExprTypeError::TypeCallTypeError => format!("TypeError: "),
            OriginalSemaExprTypeError::TodoScopeResolution => {
                format!("Type Error: TodoScopeResolution")
            }
            OriginalSemaExprTypeError::TodoBoxColon => {
                format!("Type Error: TodoBoxColon")
            }
            OriginalSemaExprTypeError::FinalDestination => {
                format!("Type Error: final destination")
            }
            OriginalSemaExprTypeError::FugitivePathTypeError => {
                format!("Type Error: form path error")
            }
            OriginalSemaExprTypeError::AmbiguousTypePath => {
                format!("Type Error: AmbiguousTypePath")
            }
            OriginalSemaExprTypeError::RitchieCallWrongNumberOfArguments {
                number_of_nonself_parameters,
                number_of_nonself_arguments,
            } => {
                format!("expected {number_of_nonself_parameters} argument, found {number_of_nonself_arguments}")
            }
            OriginalSemaExprTypeError::AmbiguousListExpr => {
                format!("Type Error: AmbiguateListExpr")
            }
            OriginalSemaExprTypeError::AmbiguousTildeExpr => {
                format!("Type Error: AmbiguateTildeExpr")
            }
            OriginalSemaExprTypeError::ExpectedCurryButGotRitchieInstead => {
                format!("Type Error: expected curry but got Ritchie instead")
            }
            OriginalSemaExprTypeError::CannotUnveil => {
                format!("Type Error: cannot unveil")
            }
            OriginalSemaExprTypeError::CannotUnwrap => {
                format!("Type Error: cannot unwrap")
            }
            OriginalSemaExprTypeError::NoConstructor => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, _ctx: &RegionDiagnosticsContext) -> TextRange {
        todo!()
        // match self {
        //     OriginalSemaExprTypeError::NoSuchField { ident_token, .. } => {
        //         ctx.token_text_range(ident_token.regional_token_idx())
        //     }
        //     OriginalSemaExprTypeError::NoMethodForType { ident_token, .. } => {
        //         ctx.token_text_range(ident_token.regional_token_idx())
        //     }
        //     _ => ctx.expr_text_range(self.0),
        // }
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

    fn range(&self, _ctx: &RegionDiagnosticsContext) -> TextRange {
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
            OriginalFluffyTermExpectationError::TypePathMismatchForSubtyping {
                expected,
                expectee,
                expected_path,
                expectee_path,
            } => format!(
                "Type Error: type path mismatch in seeing `{}` as a subtype of `{}`, expected `{}`, but got `{}` instead",
                expectee.show(ctx.db(), ctx.fluffy_term_region().terms()),
                expected.show(ctx.db(), ctx.fluffy_term_region().terms()),
                expected_path.display(ctx.db()),
                expectee_path.display(ctx.db())
            ),
            OriginalFluffyTermExpectationError::ExpectedCategory { expectee: _ } => {
                format!("Term Error: expected category",)
            }
            OriginalFluffyTermExpectationError::ExpectedFunctionType => {
                format!("Term Error: expected function type",)
            }
            OriginalFluffyTermExpectationError::ExpectedSubtype { expectee: _ } => {
                format!("Term Error: expected subtype",)
            }
            OriginalFluffyTermExpectationError::ExpectedCoersion {  expectee, expected, contract } => {
                format!(
                    "Term Error: expected coersion from `{}` to `{}` under contract `{}`",
                    expectee.show(ctx.db(), ctx.fluffy_term_region().terms()),
                    expected.show(ctx.db(), ctx.fluffy_term_region().terms()),
                    contract.as_str(),
                )
            }
            OriginalFluffyTermExpectationError::TypePathMismatchForCoersion { contract, ty_expected, expectee, expected_path, expectee_path } => format!(
                "Type Error: type path mismatch in coersing `{}` into `{}` of contract `{}`, expected `{}`, but got `{}` instead",
                expectee.show(ctx.db(), ctx.fluffy_term_region().terms()),
                ty_expected.show(ctx.db(), ctx.fluffy_term_region().terms()),
                contract.as_str(),
                expected_path.display(ctx.db()),
                expectee_path.display(ctx.db())
            ),
            OriginalFluffyTermExpectationError::Place(e) => format!("place error {}", e),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &RegionDiagnosticsContext) -> TextRange {
        ctx.expr_text_range(self.0.syn_expr_idx())
    }
}
