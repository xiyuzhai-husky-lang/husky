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
    pub fn resolve_as_much_as_possible(&mut self, db: &dyn TermDb, level: FluffyTermResolveLevel) {
        todo!()
    }
}
