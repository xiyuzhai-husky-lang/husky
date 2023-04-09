mod application;
mod place_ty;
mod ritchie;

pub use self::place_ty::*;
pub use self::ritchie::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
#[enum_class::from_variants]
pub enum FluffyTerm {
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
    /// terms with undetermined arguments
    Hollow(HollowTerm),
}

impl From<Term> for FluffyTerm {
    fn from(value: Term) -> Self {
        todo!()
    }
}
