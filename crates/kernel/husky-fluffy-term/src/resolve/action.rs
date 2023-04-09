use super::*;

pub enum FluffyTermResolveAction {
    SubstituteHole {
        hole: HollowTerm,
        substitution: FluffyTerm,
    },
    AddExpectation {
        src: ExpectationSource,
        expectee: FluffyTerm,
        expectation: FluffyTermExpectationData,
    },
}
