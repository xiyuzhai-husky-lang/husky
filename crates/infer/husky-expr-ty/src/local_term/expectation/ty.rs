use super::*;

#[derive(Debug, Clone)]
pub(crate) struct ExpectType;

impl ExpectLocalTerm for ExpectType {
    type Result = ExpectTypeResult;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

pub(crate) enum ExpectTypeResult {
    ResolvedOk(LocalTerm),
    ResolvedErr(LocalTermExpectationError),
}

impl From<ExpectType> for LocalTermExpectation {
    fn from(value: ExpectType) -> Self {
        LocalTermExpectation::Sort
    }
}

impl From<ExpectTypeResult> for LocalTermExpectationResult {
    fn from(value: ExpectTypeResult) -> Self {
        todo!()
    }
}
