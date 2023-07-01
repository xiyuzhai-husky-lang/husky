use super::*;

pub enum FluffyTermResolveAction {
    AddExpectation {
        src: ExpectationSource,
        expectee: FluffyTerm,
        expectation: Expectation,
    },
    FillHole {
        hole: Hole,
        term: FluffyTerm,
    },
    AddHoleConstraint {
        hole: Hole,
        hole_constraint: HoleConstraint,
    },
}
pub type FluffyTermResolveActions = SmallVec<[FluffyTermResolveAction; 2]>;
