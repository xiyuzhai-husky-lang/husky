mod application;
mod hole;
mod place_ty;
mod ritchie;
mod utils;

pub use self::application::*;
pub use self::hole::*;
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

impl FluffyTerm {
    pub fn ethereal(self) -> Option<Term> {
        match self {
            FluffyTerm::Solid(_) | FluffyTerm::Hollow(_) => None,
            _ => Some(unsafe { std::mem::transmute(self) }),
        }
    }
}

impl From<Term> for FluffyTerm {
    fn from(term: Term) -> Self {
        unsafe { std::mem::transmute(term) }
    }
}

#[test]
fn term_to_fluffy_term_works() {
    fn t(a: impl Copy + Into<Term> + Into<FluffyTerm>) {
        let term: Term = a.into();
        let fluffy_term: FluffyTerm = a.into();
        let term_to_fluffy_term: FluffyTerm = term.into();
        assert_eq!(fluffy_term, term_to_fluffy_term)
    }
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let term_menu = db.term_menu(toolchain);
    t(TermLiteral::I8(1))
}
