use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpectAnyDerived;

impl const ProvideEntityPathTypeExpectation for ExpectAnyDerived {
    fn entity_path_ty_expectation(&self) -> EntityPathTypeExpectation {
        todo!()
    }
}

impl ExpectLocalTerm for ExpectAnyDerived {
    type Outcome = ExpectInsSortOutcome;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}
