use super::*;

pub enum FluffyTermResolveAction {
    SubstituteHole {
        hole: Hole,
        substitution: FluffyTerm,
    },
    AddExpectation {
        src: ExpectationSource,
        expectee: FluffyTerm,
        expectation: ExpectationData,
    },
}
