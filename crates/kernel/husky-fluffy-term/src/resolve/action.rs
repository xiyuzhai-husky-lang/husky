use super::*;

pub enum FluffyTermResolveAction {
    FillHole {
        hole: Hole,
        term: FluffyTerm,
    },
    AddExpectation {
        src: ExpectationSource,
        expectee: FluffyTerm,
        expectation: Expectation,
    },
}
pub type FluffyTermResolveActions = SmallVec<[FluffyTermResolveAction; 2]>;
