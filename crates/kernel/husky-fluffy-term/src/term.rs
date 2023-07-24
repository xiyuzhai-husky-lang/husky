mod application;
mod curry;
mod hole;
mod ritchie;
mod symbol_ty;
mod utils;

use salsa::DisplayWithDb;

pub use self::application::*;
pub use self::hole::*;
pub use self::ritchie::*;
pub use self::symbol_ty::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
#[enum_class::from_variants]
pub enum FluffyTerm {
    Literal(TermLiteral),
    Symbol(EtherealTermSymbol),
    Variable(EtherealTermVariable),
    EntityPath(TermEntityPath),
    Category(TermCategory),
    Universe(TermUniverse),
    Curry(EtherealTermCurry),
    Ritchie(EtherealTermRitchie),
    Abstraction(EtherealTermAbstraction),
    Application(EtherealTermApplication),
    Subitem(EtherealTermSubitem),
    AsTraitSubitem(EtherealTermAsTraitSubitem),
    TraitConstraint(EtherealTermTraitConstraint),
    /// terms with determined local lifetimes and places, without undetermined arguments
    Solid(SolidTerm),
    /// terms with undetermined arguments
    Hollow(HollowTerm),
}

impl FluffyTerm {
    #[inline(always)]
    pub fn ethereal(self) -> Option<EtherealTerm> {
        match self {
            FluffyTerm::Solid(_) | FluffyTerm::Hollow(_) => None,
            _ => Some(unsafe { std::mem::transmute(self) }),
        }
    }
}

impl From<EtherealTerm> for FluffyTerm {
    #[inline(always)]
    fn from(term: EtherealTerm) -> Self {
        unsafe { std::mem::transmute(term) }
    }
}

#[test]
fn term_to_fluffy_term_works() {
    fn t(a: impl Copy + Into<EtherealTerm> + Into<FluffyTerm>) {
        let term: EtherealTerm = a.into();
        let fluffy_term: FluffyTerm = a.into();
        let term_to_fluffy_term: FluffyTerm = term.into();
        assert_eq!(fluffy_term, term_to_fluffy_term)
    }
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let term_menu = db.ethereal_term_menu(toolchain);
    t(TermLiteral::I8(1))
}

impl FluffyTerm {
    pub fn show(self, db: &dyn FluffyTermDb, region: &FluffyTermRegion) -> String {
        match self.nested() {
            NestedFluffyTerm::Ethereal(term) => term.display(db).to_string(),
            NestedFluffyTerm::Solid(term) => term.show(db, region.terms.solid_terms()),
            NestedFluffyTerm::Hollow(_) => todo!(),
        }
    }
}
