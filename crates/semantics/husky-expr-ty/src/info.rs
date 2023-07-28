mod variant;

pub use self::variant::*;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = ExprTypeDb)]
pub struct ExprTypeInfo {
    disambiguation_and_ty_result: ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)>,
    expectation_rule_idx: OptionFluffyTermExpectationIdx,
}

impl ExprTypeInfo {
    pub(crate) fn new(
        ty_result: ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)>,
        expectation_rule_idx: OptionFluffyTermExpectationIdx,
    ) -> Self {
        Self {
            disambiguation_and_ty_result: ty_result,
            expectation_rule_idx,
        }
    }

    pub fn ty(&self) -> ExprTypeResultRef<FluffyTerm> {
        Ok(*self.disambiguation_and_ty_result.as_ref()?.1.as_ref()?)
    }

    pub fn disambiguation(&self) -> ExprTypeResultRef<&ExprDisambiguation> {
        Ok(&self.disambiguation_and_ty_result.as_ref()?.0)
    }
}
