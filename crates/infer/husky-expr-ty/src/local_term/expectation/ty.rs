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

impl From<ExpectType> for LocalTermExpectationRuleVariant {
    fn from(value: ExpectType) -> Self {
        LocalTermExpectationRuleVariant::Sort
    }
}

impl From<ExpectTypeResult> for LocalTermExpectationResult {
    fn from(value: ExpectTypeResult) -> Self {
        todo!()
    }
}
