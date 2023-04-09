use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ExprTypeResolveProgress {
    Unresolved,
    Expected(ExpectationResolveProgress),
    Unexpected(ExprTypeResult<FluffyTerm>),
}

impl From<ExpectationResolveProgress> for ExprTypeResolveProgress {
    fn from(v: ExpectationResolveProgress) -> Self {
        Self::Expected(v)
    }
}
