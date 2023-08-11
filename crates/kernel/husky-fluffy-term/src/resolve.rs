mod action;
mod error;
mod level;

pub use self::action::*;
pub use self::error::*;
pub use self::level::*;

use crate::*;

impl FluffyTermRegion {
    fn next_effect(
        &mut self,
        db: &dyn FluffyTermDb,
        level: FluffyTermResolveLevel,
    ) -> AltOption<FluffyTermEffect> {
        for expectation in self.expectations.unresolved_expectation_iter_mut() {
            expectation.resolve(db, &mut self.terms)?
        }
        for (hole, hole_kind, constraints) in self
            .terms
            .hollow_terms()
            .empty_holes_with_non_empty_constraints()
        {
            for constraint in constraints {
                match constraint {
                    HoleConstraint::CoercibleFrom { target } => {
                        debug_assert_ne!(Into::<FluffyTerm>::into(hole), *target);
                        // todo: check HoleKind?
                        return AltSome(FluffyTermEffect {
                            subsequent_actions: smallvec![FluffyTermResolveAction::FillHole {
                                hole,
                                term: *target
                            }],
                        });
                    }
                    HoleConstraint::CoercibleInto { target } => {
                        debug_assert_ne!(Into::<FluffyTerm>::into(hole), *target);
                        // todo: check HoleKind?
                        return AltSome(FluffyTermEffect {
                            subsequent_actions: smallvec![FluffyTermResolveAction::FillHole {
                                hole,
                                term: *target
                            }],
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

    pub fn resolve_as_much_as_possible(
        &mut self,
        db: &dyn FluffyTermDb,
        level: FluffyTermResolveLevel,
    ) {
        while let AltOption::AltSome(effect) = self.next_effect(db, level) {
            for action in effect.take_subsequent_actions() {
                match action {
                    FluffyTermResolveAction::AddExpectation {
                        src,
                        expectee,
                        expectation,
                    } => {
                        self.add_expectation(src, expectee, expectation);
                    }
                    FluffyTermResolveAction::FillHole { hole, term } => {
                        self.terms_mut().fill_hole(db, hole, term)
                    }
                    FluffyTermResolveAction::AddHoleConstraint {
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
