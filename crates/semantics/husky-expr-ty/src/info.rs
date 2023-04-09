mod disambiguation;
mod progress;

pub use self::disambiguation::*;

pub(crate) use self::progress::*;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub struct ExprTypeInfo {
    ty_result: ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)>,
    expectation_rule_idx: OptionFluffyTermExpectationIdx,
    resolve_progress: ExprTypeResolveProgress,
}

impl ExprTypeInfo {
    pub(crate) fn new(
        ty_result: ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)>,
        expectation_rule_idx: OptionFluffyTermExpectationIdx,
    ) -> Self {
        Self {
            ty_result,
            expectation_rule_idx,
            resolve_progress: ExprTypeResolveProgress::Unresolved,
        }
    }

    pub(crate) fn finalize(&mut self, unresolved_term_table: &FluffyTermRegion) {
        todo!()
        // let Ok(ty) = self.ty() else { return };
        // self.resolve_progress = match self.expectation_rule_idx.into_option() {
        //     Some(expectation_rule_idx) => unresolved_term_table[expectation_rule_idx]
        //         .resolve_progress()
        //         .duplicate(expectation_rule_idx)
        //         .into(),
        //     None => match ty {
        //         FluffyTerm::Term(term) => ExprTypeResolveProgress::Unexpected(Ok(term.into())),
        //         FluffyTerm::Unresolved(ty) => ExprTypeResolveProgress::Unexpected(Err(
        //             DerivedExprTypeError::UnresolvedLocalTerm.into(),
        //         )),
        //         _ => todo!(),
        //     },
        // }
    }

    pub(crate) fn resolve_progress(&self) -> &ExprTypeResolveProgress {
        &self.resolve_progress
    }

    pub fn ty(&self) -> ExprTypeResultRef<FluffyTerm> {
        Ok(*self.ty_result.as_ref()?.1.as_ref()?)
    }

    pub fn disambiguation(&self) -> ExprTypeResultRef<ExprDisambiguation> {
        Ok(self.ty_result.as_ref()?.0)
    }
}
