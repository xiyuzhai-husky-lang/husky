use super::*;

#[derive(Debug)]
pub struct FillHole {
    pub(super) hole: Hole,
    pub(super) term: FlyTerm,
}

#[derive(Debug)]
pub enum FlyTermResolveAction {
    AddExpectation {
        src: ExpectationSource,
        expectee: FlyTerm,
        expectation: Expectation,
    },
    AddHoleConstraint {
        hole: Hole,
        hole_constraint: HoleConstraint,
    },
    FillHole(FillHole),
}
pub type FlyTermResolveActions = SmallVec<[FlyTermResolveAction; 2]>;
