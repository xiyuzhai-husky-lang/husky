use husky_print_utils::p;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub struct ExprTypeRegion {
    path: RegionPath,
    expr_ty_infos: ExprMap<ExprTypeInfo>,
    unresolved_term_table: UnresolvedTermTable,
}

impl ExprTypeRegion {
    pub(crate) fn new(
        db: &dyn ExprTypeDb,
        reduced_term_menu: ReducedTermMenu,
        path: RegionPath,
        mut expr_ty_infos: ExprMap<ExprTypeInfo>,
        mut unresolved_term_table: UnresolvedTermTable,
    ) -> Self {
        expr_ty_infos
            .iter_mut()
            .for_each(|info| info.finalize(&unresolved_term_table));
        Self {
            path,
            expr_ty_infos,
            unresolved_term_table,
        }
    }
}

impl std::ops::Index<ExprIdx> for ExprTypeRegion {
    type Output = ExprTypeInfo;

    fn index(&self, index: ExprIdx) -> &Self::Output {
        &self.expr_ty_infos[index]
    }
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

    fn finalize(&mut self, unresolved_term_table: &UnresolvedTermTable) {
        let Ok(ty) = self.ty_result else { return };
        self.resolve_progress = match self.expectation_rule_idx.into_option() {
            Some(expectation_rule_idx) => unresolved_term_table[expectation_rule_idx]
                .resolve_progress()
                .duplicate(expectation_rule_idx),
            None => match ty {
                LocalTerm::Resolved(term) => {
                    LocalTermExpectationResolveProgress::Resolved(LocalTermExpectationResolved {
                        implicit_conversion: LocalTermImplicitConversion::None,
                        local_term: term.into(),
                    })
                }
                LocalTerm::Unresolved(ty) => LocalTermExpectationResolveProgress::Err(
                    DerivedLocalTermExpectationResolveError::UnresolvedLocalTerm.into(),
                ),
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
