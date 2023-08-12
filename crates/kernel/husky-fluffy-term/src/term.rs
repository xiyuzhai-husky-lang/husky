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
#[salsa::debug_with_db(db = FluffyTermDb)]
pub struct FluffyTerm {
    place: Option<Place>,
    base: FluffyTermBase,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = FluffyTermDb)]
#[enum_class::from_variants]
pub enum FluffyTermBase {
    Ethereal(EtherealTerm),
    Solid(SolidTerm),
    Hollow(HollowTerm),
}

impl From<EtherealTerm> for FluffyTerm {
    #[inline(always)]
    fn from(term: EtherealTerm) -> Self {
        Self {
            place: None,
            base: term.into(),
        }
    }
}

impl From<TermLiteral> for FluffyTerm {
    fn from(value: TermLiteral) -> Self {
        Into::<EtherealTerm>::into(value).into()
    }
}

impl From<TermEntityPath> for FluffyTerm {
    fn from(value: TermEntityPath) -> Self {
        Into::<EtherealTerm>::into(value).into()
    }
}

impl From<EtherealTermApplication> for FluffyTerm {
    fn from(value: EtherealTermApplication) -> Self {
        Into::<EtherealTerm>::into(value).into()
    }
}

impl From<EtherealTermCurry> for FluffyTerm {
    fn from(value: EtherealTermCurry) -> Self {
        Into::<EtherealTerm>::into(value).into()
    }
}

impl From<TermCategory> for FluffyTerm {
    fn from(value: TermCategory) -> Self {
        Into::<EtherealTerm>::into(value).into()
    }
}

impl From<EtherealTermSymbol> for FluffyTerm {
    fn from(value: EtherealTermSymbol) -> Self {
        Into::<EtherealTerm>::into(value).into()
    }
}

impl From<EtherealTermVariable> for FluffyTerm {
    fn from(value: EtherealTermVariable) -> Self {
        Into::<EtherealTerm>::into(value).into()
    }
}

impl From<EtherealTermRitchie> for FluffyTerm {
    fn from(value: EtherealTermRitchie) -> Self {
        Into::<EtherealTerm>::into(value).into()
    }
}

impl From<SolidTerm> for FluffyTerm {
    #[inline(always)]
    fn from(term: SolidTerm) -> Self {
        Self {
            place: None,
            base: term.into(),
        }
    }
}

impl From<HollowTerm> for FluffyTerm {
    #[inline(always)]
    fn from(term: HollowTerm) -> Self {
        Self {
            place: None,
            base: term.into(),
        }
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
    pub fn place(self) -> Option<Place> {
        self.place
    }

    pub fn base(self) -> FluffyTermBase {
        self.base
    }

    pub fn show(self, db: &dyn FluffyTermDb, terms: &FluffyTerms) -> String {
        self.data_inner(db, terms).show(db, terms)
    }
}
