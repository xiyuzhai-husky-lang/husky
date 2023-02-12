use husky_print_utils::p;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub struct ExprTypeRegion {
    path: RegionPath,
    expr_ty_infos: ExprMap<ExprTypeInfo>,
    expr_local_terms: ExprMap<ExprTermResult<LocalTerm>>,
    inherited_symbol_tys: InheritedSymbolMap<ReducedTerm>,
    current_symbol_tys: CurrentSymbolMap<LocalTerm>,
    unresolved_term_table: LocalTermTable,
    return_ty: Option<ReducedTerm>,
    self_ty: Option<ReducedTerm>,
}

impl ExprTypeRegion {
    pub(crate) fn new(
        db: &dyn ExprTypeDb,
        reduced_term_menu: ReducedTermMenu,
        path: RegionPath,
        mut expr_ty_infos: ExprMap<ExprTypeInfo>,
        expr_terms: ExprMap<ExprTermResult<LocalTerm>>,
        inherited_symbol_tys: InheritedSymbolMap<ReducedTerm>,
        current_symbol_tys: CurrentSymbolMap<LocalTerm>,
        mut unresolved_term_table: LocalTermTable,
        return_ty: Option<ReducedTerm>,
        self_ty: Option<ReducedTerm>,
    ) -> Self {
        expr_ty_infos
            .iter_mut()
            .for_each(|info| info.finalize(&unresolved_term_table));
        Self {
            path,
            expr_ty_infos,
            expr_local_terms: expr_terms,
            inherited_symbol_tys,
            current_symbol_tys,
            unresolved_term_table,
            return_ty,
            self_ty,
        }
    }

    // pub fn expr_local_term(
    //     &self,
    //     expr_idx: ExprIdx,
    // ) -> Option<Result<Result<ReducedTerm, &LocalTermResolveError>, &ExprTermError>> {
    //     self.expr_local_terms.get(expr_idx).map(|expr_term| {
    //         expr_term
    //             .as_ref()
    //             .map(|local_term| self.resolved_term(*local_term))
    //     })
    // }

    // fn resolved_term(&self, local_term: LocalTerm) -> Result<ReducedTerm, &LocalTermResolveError> {
    //     match local_term {
    //         LocalTerm::Resolved(term) => Ok(term),
    //         LocalTerm::Unresolved(term) => {
    //             todo!()
    //         }
    //     }
    // }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub struct ExprTypeInfo {
    ty_result: ExprTypeResult<LocalTerm>,
    expectation_rule_idx: OptionLocalTermExpectationRuleIdx,
    resolve_progress: LocalTermExpectationResolveProgress,
}

impl ExprTypeInfo {
    pub(crate) fn new(
        ty_result: ExprTypeResult<LocalTerm>,
        expectation_rule_idx: OptionLocalTermExpectationRuleIdx,
    ) -> Self {
        Self {
            ty_result,
            expectation_rule_idx,
            resolve_progress: LocalTermExpectationResolveProgress::Unresolved,
        }
    }

    pub(crate) fn ty(&self) -> ExprTypeResultRef<LocalTerm> {
        self.ty_result.as_ref().copied()
    }

    fn finalize(&mut self, unresolved_term_table: &LocalTermTable) {
        let Ok(ty) = self.ty_result else { return };
        self.resolve_progress = match self.expectation_rule_idx.into_option() {
            Some(expectation_rule_idx) => unresolved_term_table[expectation_rule_idx]
                .resolve_progress()
                .duplicate(expectation_rule_idx),
            None => match ty {
                LocalTerm::Resolved(term) => LocalTermExpectationResolveProgress::Resolved(
                    LocalTermExpectationResult::OkNoExpectation {
                        local_term: term.into(),
                        implicit_conversion: LocalTermImplicitConversion::None,
                    },
                ),
                LocalTerm::Unresolved(ty) => {
                    LocalTermExpectationResolveProgress::Resolved(LocalTermExpectationResult::Err(
                        DerivedLocalTermExpectationError::UnresolvedLocalTerm.into(),
                    ))
                }
            },
        }
    }

    pub(crate) fn resolve_progress(&self) -> &LocalTermExpectationResolveProgress {
        &self.resolve_progress
    }
}

#[salsa::tracked(jar = ExprTypeJar, return_ref)]
pub(crate) fn expr_ty_region(db: &dyn ExprTypeDb, expr_region: ExprRegion) -> ExprTypeRegion {
    let mut engine = ExprTypeEngine::new(db, expr_region);
    engine.infer_all();
    engine.finish()
}

pub(crate) struct PatternExprTypeInfo {
    ty: PatternExprTypeResult<LocalTerm>,
}

impl PatternExprTypeInfo {
    pub(crate) fn new(ty: PatternExprTypeResult<LocalTerm>) -> Self {
        Self { ty }
    }

    pub(crate) fn ty(&self) -> Result<&LocalTerm, &PatternExprTypeError> {
        self.ty.as_ref()
    }
}

pub(crate) struct PatternSymbolTypeInfo {
    ty: PatternSymbolTypeResult<LocalTerm>,
}

impl PatternSymbolTypeInfo {
    pub(crate) fn new(ty: PatternSymbolTypeResult<LocalTerm>) -> Self {
        Self { ty }
    }
}
