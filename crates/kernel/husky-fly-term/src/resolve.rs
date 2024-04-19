mod action;
mod error;
mod level;

pub use self::action::*;
pub use self::error::*;
pub use self::level::*;

use crate::*;

impl FlyTermRegion {
    fn next_effect(&mut self, db: &::salsa::Db) -> AltOption<FlyTermEffect> {
        for expectation in self.expectations.unresolved_expectation_iter_mut() {
            expectation.resolve(db, &mut self.terms)?
        }
        for (hole, hole_kind, constraints) in self
            .terms
            .hol_terms()
            .empty_holes_with_non_empty_constraints()
        {
            for constraint in constraints {
                match constraint {
                    HoleConstraint::CoercibleFrom { target } => {
                        debug_assert_ne!(Into::<FlyTerm>::into(hole), *target);
                        // todo: check HoleKind?
                        return AltSome(FlyTermEffect {
                            subsequent_actions: smallvec![FlyTermResolveAction::FillHole(
                                FillHole {
                                    hole,
                                    term: *target
                                }
                            )],
                        });
                    }
                    HoleConstraint::CoercibleInto { target } => {
                        debug_assert_ne!(Into::<FlyTerm>::into(hole), *target);
                        // todo: check HoleKind?
                        return AltSome(FlyTermEffect {
                            subsequent_actions: smallvec![FlyTermResolveAction::FillHole(
                                FillHole {
                                    hole,
                                    term: *target
                                }
                            )],
                        });
                    }
                    HoleConstraint::Subtype { target } => {
                        if Into::<FlyTerm>::into(hole) == *target {
                            return AltNone;
                        }
                        // todo: check HoleKind?
                        // todo: return AltNone if target is not resolved?
                        return AltSome(FlyTermEffect {
                            subsequent_actions: smallvec![FlyTermResolveAction::FillHole(
                                FillHole {
                                    hole,
                                    term: *target
                                }
                            )],
                        });
                    }
                    HoleConstraint::Supertype { target } => {
                        if Into::<FlyTerm>::into(hole) == *target {
                            return AltNone;
                        }
                        // todo: check HoleKind?
                        // todo: return AltNone if target is not resolved?
                        return AltSome(FlyTermEffect {
                            subsequent_actions: smallvec![FlyTermResolveAction::FillHole(
                                FillHole {
                                    hole,
                                    term: *target
                                }
                            )],
                        });
                    }
                }
            }
            match constraints.len() {
                0 => unreachable!(),
                1 => todo!(),
                _ => todo!(),
            }
        }
        AltOption::AltNone
    }

    pub fn resolve_as_much_as_possible(&mut self, db: &::salsa::Db) {
        while let AltOption::AltSome(effect) = self.next_effect(db) {
            for action in effect.take_subsequent_actions() {
                match action {
                    FlyTermResolveAction::AddExpectation {
                        src,
                        expectee,
                        expectation,
                    } => {
                        self.add_expectation(src, expectee, expectation, db);
                    }
                    FlyTermResolveAction::FillHole(FillHole { hole, term }) => {
                        self.terms_mut().fill_hole(db, hole, term)
                    }
                    FlyTermResolveAction::AddHoleConstraint {
                        hole,
                        hole_constraint,
                    } => self
                        .hollow_terms_mut()
                        .add_hole_constraint(hole, hole_constraint),
                }
            }
        }
    }
}
