use super::*;

pub enum FluffyTermResolveAction {
    FillHole {
        hole: Hole,
        term: FluffyTerm,
    },
    AddExpectation {
        src: ExpectationSource,
        expectee: FluffyTerm,
        expectation: ExpectationData,
    },
}
