use super::*;

#[derive(Debug)]
pub enum FlyTermResolveAction {
    AddExpectation {
        src: ExpectationSource,
        expectee: FlyTerm,
        expectation: Expectation,
    },
    #[deprecated(note = "use AddHoleConstraint instead unless necessary")]
    FillHole { hole: Hole, term: FlyTerm },
    AddHoleConstraint {
        hole: Hole,
        hole_constraint: HoleConstraint,
    },
}
pub type FlyTermResolveActions = SmallVec<[FlyTermResolveAction; 2]>;
