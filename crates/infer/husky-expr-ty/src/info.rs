mod disambiguation;
mod progress;

pub use self::disambiguation::*;
pub use self::progress::*;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub struct ExprTypeInfo {
    ty_result: ExprTypeResult<LocalTerm>,
    disambiguation: ExprTypeResult<ExprDisambiguation>,
    expectation_rule_idx: OptionLocalTermExpectationIdx,
    resolve_progress: ExprTypeResolveProgress,
}

impl ExprTypeInfo {
    pub(crate) fn new(
        ty_result: ExprTypeResult<LocalTerm>,
        disambiguation: ExprTypeResult<ExprDisambiguation>,
        expectation_rule_idx: OptionLocalTermExpectationIdx,
    ) -> Self {
        Self {
            ty_result,
            disambiguation,
            expectation_rule_idx,
            resolve_progress: ExprTypeResolveProgress::Unresolved,
        }
    }

    pub(crate) fn ty(&self) -> ExprTypeResultRef<LocalTerm> {
        self.ty_result.as_ref().copied()
    }

    pub(crate) fn finalize(&mut self, unresolved_term_table: &LocalTermRegion) {
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

    pub fn ty_result(&self) -> Result<&LocalTerm, &ExprTypeError> {
        self.ty_result.as_ref()
    }

    pub fn disambiguation(&self) -> ExprTypeResultRef<ExprDisambiguation> {
        self.disambiguation.as_ref().copied()
    }
}
