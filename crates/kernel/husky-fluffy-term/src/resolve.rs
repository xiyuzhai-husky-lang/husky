mod action;
mod error;
mod level;

pub use self::action::*;
pub use self::error::*;
pub use self::level::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
#[enum_class::from_variants]
pub enum ResolvedTerm {
    Literal(TermLiteral),
    Symbol(TermSymbol),
    Hole(TermPlaceholder),
    EntityPath(TermEntityPath),
    Category(TermCategory),
    Universe(TermUniverse),
    Curry(TermCurry),
    Ritchie(TermRitchie),
    Abstraction(TermAbstraction),
    Application(TermApplication),
    Subentity(TermSubentity),
    AsTraitSubentity(TermAsTraitSubentity),
    TraitConstraint(TermTraitConstraint),
    /// terms with determined local lifetimes and places, without undetermined arguments
    Solid(SolidTerm),
}

impl FluffyTerm {
    pub fn resolve(self) -> Either<ResolvedTerm, HollowTerm> {
        todo!()
    }
}

impl FluffyTermRegion {
    fn next_expectation_effect(
        &mut self,
        db: &dyn FluffyTermDb,
        level: FluffyTermResolveLevel,
    ) -> Option<(FluffyTermExpectationIdx, FluffyTermExpectationEffect)> {
        for (idx, rule) in self.expectations().unresolved_rule_iter() {
            todo!()
            // if let Some(action) = rule.resolve_expectation(db, &mut self.interner, level) {
            //     return Some((idx, action));
            // }
        }
        None
    }

    pub fn resolve_as_much_as_possible(
        &mut self,
        db: &dyn FluffyTermDb,
        level: FluffyTermResolveLevel,
    ) {
        while let Some((rule_idx, effect)) = self.next_expectation_effect(db, level) {
            if let Some(actions) = self.expectations_mut().take_effect(rule_idx, effect) {
                for action in actions {
                    match action {
                        FluffyTermResolveAction::SubstituteHole { hole, substitution } => {
                            todo!()
                            // self.substitute_hole(db, implicit_symbol, substitution)
                        }
                        FluffyTermResolveAction::AddExpectation {
                            src,
                            expectee,
                            expectation,
                        } => {
                            self.add_expectation_rule(src, expectee, expectation);
                        }
                    }
                }
            }
        }
    }
}
