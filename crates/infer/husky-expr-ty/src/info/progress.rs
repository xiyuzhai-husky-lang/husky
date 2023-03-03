use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ExprTypeResolveProgress {
    Unresolved,
    Expected(LocalTermExpectationResolveProgress),
    Unexpected(ExprTypeResult<LocalTerm>),
}

impl From<LocalTermExpectationResolveProgress> for ExprTypeResolveProgress {
    fn from(v: LocalTermExpectationResolveProgress) -> Self {
        Self::Expected(v)
    }
}
