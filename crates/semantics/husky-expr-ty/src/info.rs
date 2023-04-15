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
}

impl ExprTypeInfo {
    pub(crate) fn new(
        ty_result: ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)>,
        expectation_rule_idx: OptionFluffyTermExpectationIdx,
    ) -> Self {
        Self {
            ty_result,
            expectation_rule_idx,
        }
    }

    pub fn ty(&self) -> ExprTypeResultRef<FluffyTerm> {
        Ok(*self.ty_result.as_ref()?.1.as_ref()?)
    }

    pub fn disambiguation(&self) -> ExprTypeResultRef<&ExprDisambiguation> {
        Ok(&self.ty_result.as_ref()?.0)
    }
}
