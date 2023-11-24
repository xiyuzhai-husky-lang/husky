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
    place: Option<FluffyPlace>,
    base: FluffyTermBase,
}

impl FluffyTerm {
    pub(crate) fn new_ethereal(place: FluffyPlace, ethereal_term: EtherealTerm) -> Self {
        Self {
            place: Some(place),
            base: ethereal_term.into(),
        }
    }

    pub(crate) fn with_place(self, place: FluffyPlace) -> Self {
        Self {
            place: Some(place),
            base: self.base,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = FluffyTermDb)]
#[enum_class::from_variants]
pub enum FluffyTermBase {
    Ethereal(EtherealTerm),
    Solid(SolidTerm),
    Hollow(HollowTerm),
    Place,
}

impl From<FluffyPlace> for FluffyTerm {
    fn from(place: FluffyPlace) -> Self {
        FluffyTerm {
            place: Some(place),
            base: FluffyTermBase::Place,
        }
    }
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

impl From<EtherealTermRune> for FluffyTerm {
    fn from(value: EtherealTermRune) -> Self {
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
    pub fn place(self) -> Option<FluffyPlace> {
        self.place
    }

    pub fn base_resolved(self, engine: &impl FluffyTermEngine) -> FluffyTermBase {
        self.base_resolved_inner(engine.fluffy_terms())
    }

    pub fn base_resolved_inner(
        self,
        terms: &impl std::borrow::Borrow<HollowTerms>,
    ) -> FluffyTermBase {
        match self.base {
            FluffyTermBase::Ethereal(_) | FluffyTermBase::Solid(_) => self.base,
            FluffyTermBase::Hollow(term) => match term.resolve_progress(terms.borrow()) {
                TermResolveProgress::UnresolvedHollow => self.base,
                TermResolveProgress::ResolvedEthereal(term) => term.into(),
                TermResolveProgress::ResolvedSolid(term) => term.into(),
                TermResolveProgress::Err => todo!(),
            },
            FluffyTermBase::Place => self.base,
        }
    }

    pub fn show(self, db: &dyn FluffyTermDb, terms: &FluffyTerms) -> String {
        self.data_inner(db, terms).show(db, terms)
    }

    pub(crate) fn base(&self) -> FluffyTermBase {
        self.base
    }
}
