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
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub struct ExprTypeInfo {
    ty_result: ExprTypeResult<LocalTerm>,
    expectation_rule_idx: OptionLocalTermExpectationIdx,
    resolve_progress: ExprTypeResolveProgress,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ExprTypeResolveProgress {
    Unresolved,
    Expected(LocalTermExpectationResolveProgress),
    Unexpected(ExprTypeResult<LocalTerm>),
}

impl From<LocalTermExpectationResolveProgress> for ExprTypeResolveProgress {
    fn from(v: LocalTermExpectationResolveProgress) -> Self {
        Self::Expected(v)
    }
}

impl ExprTypeInfo {
    pub(crate) fn new(
        ty_result: ExprTypeResult<LocalTerm>,
        expectation_rule_idx: OptionLocalTermExpectationIdx,
    ) -> Self {
        Self {
            ty_result,
            expectation_rule_idx,
            resolve_progress: ExprTypeResolveProgress::Unresolved,
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
                .duplicate(expectation_rule_idx)
                .into(),
            None => match ty {
                LocalTerm::Resolved(term) => ExprTypeResolveProgress::Unexpected(Ok(term.into())),
                LocalTerm::Unresolved(ty) => ExprTypeResolveProgress::Unexpected(Err(
                    DerivedExprTypeError::UnresolvedLocalTerm.into(),
                )),
            },
        }
    }

    pub(crate) fn resolve_progress(&self) -> &ExprTypeResolveProgress {
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
