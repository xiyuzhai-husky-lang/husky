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
    ) -> Option<(FluffyTermExpectationIdx, FluffyTermExpectationEffect)> {
        for (idx, rule) in self.expectations.unresolved_rule_iter() {
            if let Some(action) = rule.resolve_expectation(db, &mut self.terms, idx, level) {
                return Some((idx, action));
            }
        }
        None
    }

    pub fn resolve_as_much_as_possible(
        &mut self,
        db: &dyn FluffyTermDb,
        level: FluffyTermResolveLevel,
    ) {
        while let Some((expectation_idx, effect)) = self.next_expectation_effect(db, level) {
            if let Some(actions) = self.expectations.take_effect(expectation_idx, effect) {
                for action in actions {
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
}
