mod action;
mod error;
mod level;

pub use self::action::*;
pub use self::error::*;
pub use self::level::*;

use crate::*;

impl FluffyTermRegion {
    fn next_expectation_effect(
        &mut self,
        db: &dyn FluffyTermDb,
        level: FluffyTermResolveLevel,
    ) -> AltOption<ExpectationEffect> {
        for expectation in self.expectations.unresolved_expectation_iter_mut() {
            expectation.resolve(db, &mut self.terms)?
        }
        AltOption::AltNone
    }

    pub fn resolve_as_much_as_possible(
        &mut self,
        db: &dyn FluffyTermDb,
        level: FluffyTermResolveLevel,
    ) {
        while let AltOption::AltSome(effect) = self.next_expectation_effect(db, level) {
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
                        self.hollow_terms_mut().fill_hole(db, hole, term)
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
