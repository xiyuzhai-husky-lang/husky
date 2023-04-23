mod variant;

pub use self::variant::*;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub struct ExprTypeInfo {
    variant_and_ty_result: ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)>,
    expectation_rule_idx: OptionFluffyTermExpectationIdx,
}

impl ExprTypeInfo {
    pub(crate) fn new(
        ty_result: ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)>,
        expectation_rule_idx: OptionFluffyTermExpectationIdx,
    ) -> Self {
        Self {
            variant_and_ty_result: ty_result,
            expectation_rule_idx,
        }
    }

    pub fn ty(&self) -> ExprTypeResultRef<FluffyTerm> {
        Ok(*self.variant_and_ty_result.as_ref()?.1.as_ref()?)
    }

    pub fn variant(&self) -> ExprTypeResultRef<&ExprDisambiguation> {
        Ok(&self.variant_and_ty_result.as_ref()?.0)
    }
}
