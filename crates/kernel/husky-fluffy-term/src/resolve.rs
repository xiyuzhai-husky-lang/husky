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
    ) -> Option<ExpectationEffect> {
        for expectation in self.expectations.unresolved_expectation_iter_mut() {
            if let Some(effect) = expectation.resolve(db, &mut self.terms) {
                return Some(effect);
            }
        }
        None
    }

    pub fn resolve_as_much_as_possible(
        &mut self,
        db: &dyn FluffyTermDb,
        level: FluffyTermResolveLevel,
    ) {
        while let Some(effect) = self.next_expectation_effect(db, level) {
            for action in effect.take_subsequent_actions() {
                match action {
                    FluffyTermResolveAction::FillHole { hole, term } => {
                        self.hollow_terms_mut().fill_hole(db, hole, term)
                    }
                    FluffyTermResolveAction::AddExpectation {
                        src,
                        expectee,
                        expectation,
                    } => {
                        self.add_expectation(src, expectee, expectation);
                    }
                }
            }
        }
    }
}
