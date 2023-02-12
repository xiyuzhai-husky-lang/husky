use super::*;

#[derive(Debug, Clone)]
pub(crate) struct ExpectSort;

impl ExpectLocalTerm for ExpectSort {
    type Result = ExpectSortResult;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

pub(crate) enum ExpectSortResult {
    ResolvedOk(LocalTerm),
    ResolvedErr(LocalTermExpectationError),
}

impl From<ExpectSort> for LocalTermExpectationRuleVariant {
    fn from(value: ExpectSort) -> Self {
        LocalTermExpectationRuleVariant::Sort
    }
}

impl From<ExpectSortResult> for LocalTermExpectationResult {
    fn from(value: ExpectSortResult) -> Self {
        todo!()
    }
}
