use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ExprTypeResolveProgress {
    Unresolved,
    Expected(FluffyTermExpectationResolveProgress),
    Unexpected(ExprTypeResult<FluffyTerm>),
}

impl From<FluffyTermExpectationResolveProgress> for ExprTypeResolveProgress {
    fn from(v: FluffyTermExpectationResolveProgress) -> Self {
        Self::Expected(v)
    }
}
