mod disambiguation;
mod progress;

pub use self::disambiguation::*;
pub use self::progress::*;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTermDb)]
pub struct ExprTypeInfo {
    ty_result: ExprTypeResult<(ExprDisambiguation, ExprTypeResult<LocalTerm>)>,
    expectation_rule_idx: OptionLocalTermExpectationIdx,
    resolve_progress: ExprTypeResolveProgress,
}

impl ExprTypeInfo {
    pub(crate) fn new(
        ty_result: ExprTypeResult<(ExprDisambiguation, ExprTypeResult<LocalTerm>)>,
        expectation_rule_idx: OptionLocalTermExpectationIdx,
    ) -> Self {
        Self {
            ty_result,
            expectation_rule_idx,
            resolve_progress: ExprTypeResolveProgress::Unresolved,
        }
    }

    pub(crate) fn finalize(&mut self, unresolved_term_table: &LocalTermRegion) {
        let Ok(ty) = self.ty() else { return };
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

    pub fn ty(&self) -> ExprTypeResultRef<LocalTerm> {
        Ok(*self.ty_result.as_ref()?.1.as_ref()?)
    }

    pub fn disambiguation(&self) -> ExprTypeResultRef<ExprDisambiguation> {
        Ok(self.ty_result.as_ref()?.0)
    }
}
