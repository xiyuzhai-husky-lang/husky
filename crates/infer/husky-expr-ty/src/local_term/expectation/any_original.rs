use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpectAnyOriginal;

impl const ProvideEntityPathTypeExpectation for ExpectAnyOriginal {
    fn entity_path_ty_expectation(&self) -> EntityPathTypeExpectation {
        todo!()
    }
}

impl ExpectLocalTerm for ExpectAnyOriginal {
    type Outcome = ExpectInsSortOutcome;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}
