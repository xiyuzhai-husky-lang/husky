use husky_print_utils::p;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub struct ExprTypeRegion {
    path: RegionPath,
    expr_ty_infos: ExprMap<TypeInfo>,
    unresolved_term_table: UnresolvedTermTable,
}

impl ExprTypeRegion {
    pub(crate) fn new(
        path: RegionPath,
        mut expr_ty_infos: ExprMap<TypeInfo>,
        mut unresolved_term_table: UnresolvedTermTable,
    ) -> Self {
        unresolved_term_table.finalize();
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
    type Output = TypeInfo;

    fn index(&self, index: ExprIdx) -> &Self::Output {
        &self.expr_ty_infos[index]
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub struct TypeInfo {
    ty_result: ExprTypeResult<LocalTerm>,
    expectation_rule: OptionExpectationIdx,
    resolve_progress: LocalTermResolveProgress,
}

impl TypeInfo {
    pub(crate) fn new(
        ty_result: ExprTypeResult<LocalTerm>,
        expectation_rule: OptionExpectationIdx,
    ) -> Self {
        Self {
            ty_result,
            expectation_rule,
            resolve_progress: LocalTermResolveProgress::Unresolved,
        }
    }

    pub(crate) fn ty(&self) -> ExprTypeResultRef<LocalTerm> {
        self.ty_result.as_ref().copied()
    }

    fn finalize(&mut self, unresolved_term_table: &UnresolvedTermTable) {
        let Ok(ty) = self.ty_result else { return };
        self.resolve_progress = match self.expectation_rule.into_option() {
            Some(expectation_rule) => unresolved_term_table[expectation_rule]
                .resolve_progress()
                .duplicate(),
            None => match ty {
                LocalTerm::Resolved(term) => LocalTermResolveProgress::Resolved {
                    implicit_conversion: LocalTermImplicitConversion::None,
                    term,
                },
                LocalTerm::Unresolved(ty) => {
                    LocalTermResolveProgress::Err(DerivedExprTypeError::UnresolvedLocalTerm.into())
                }
            },
        }
    }

    pub(crate) fn resolve_progress(&self) -> &LocalTermResolveProgress {
        &self.resolve_progress
    }
}

#[salsa::tracked(jar = ExprTypeJar, return_ref)]
pub(crate) fn expr_ty_region(db: &dyn ExprTypeDb, expr_region: ExprRegion) -> ExprTypeRegion {
    let mut engine = ExprTypeEngine::new(db, expr_region);
    engine.infer_all();
    engine.finish()
}
