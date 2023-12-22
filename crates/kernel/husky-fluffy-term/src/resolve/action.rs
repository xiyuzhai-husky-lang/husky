use super::*;

#[derive(Debug)]
pub enum FluffyTermResolveAction {
    AddExpectation {
        src: ExpectationSource,
        expectee: FluffyTerm,
        expectation: Expectation,
    },
    #[deprecated(note = "use AddHoleConstraint instead")]
    FillHole { hole: Hole, term: FluffyTerm },
    AddHoleConstraint {
        hole: Hole,
        hole_constraint: HoleConstraint,
    },
}
pub type FluffyTermResolveActions = SmallVec<[FluffyTermResolveAction; 2]>;
