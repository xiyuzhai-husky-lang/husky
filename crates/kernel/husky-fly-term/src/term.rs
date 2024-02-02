mod application;
mod curry;
mod hole;
mod ritchie;
pub mod rune;
pub mod symbol_ty;
mod utils;

pub use self::hole::*;
pub use self::ritchie::*;
pub use self::rune::*;
pub use self::symbol_ty::*;

use crate::*;
use husky_eth_term::term::{
    application::EthApplication, curry::EthCurry, ritchie::EthRitchie, rune::EthRune,
    symbol::EthSymbol,
};
use husky_term_prelude::literal::Literal;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FlyTerm {
    place: Option<FlyPlace>,
    base: FlyTermBase,
}

impl FlyTerm {
    pub(crate) fn new_ethereal(place: FlyPlace, ethereal_term: EthTerm) -> Self {
        Self {
            place: Some(place),
            base: ethereal_term.into(),
        }
    }

    pub fn with_place(self, place: FlyPlace) -> Self {
        Self {
            place: Some(place),
            base: self.base,
        }
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum FlyTermBase {
    Eth(EthTerm),
    Sol(SolTerm),
    Hol(HolTerm),
    Place,
}

impl From<FlyPlace> for FlyTerm {
    fn from(place: FlyPlace) -> Self {
        FlyTerm {
            place: Some(place),
            base: FlyTermBase::Place,
        }
    }
}

impl From<EthTerm> for FlyTerm {
    #[inline(always)]
    fn from(term: EthTerm) -> Self {
        Self {
            place: None,
            base: term.into(),
        }
    }
}

impl From<Literal> for FlyTerm {
    fn from(value: Literal) -> Self {
        Into::<EthTerm>::into(value).into()
    }
}

impl From<ItemPathTerm> for FlyTerm {
    fn from(value: ItemPathTerm) -> Self {
        Into::<EthTerm>::into(value).into()
    }
}

impl From<EthApplication> for FlyTerm {
    fn from(value: EthApplication) -> Self {
        Into::<EthTerm>::into(value).into()
    }
}

impl From<EthCurry> for FlyTerm {
    fn from(value: EthCurry) -> Self {
        Into::<EthTerm>::into(value).into()
    }
}

impl From<Category> for FlyTerm {
    fn from(value: Category) -> Self {
        Into::<EthTerm>::into(value).into()
    }
}

impl From<EthSymbol> for FlyTerm {
    fn from(value: EthSymbol) -> Self {
        Into::<EthTerm>::into(value).into()
    }
}

impl From<EthRune> for FlyTerm {
    fn from(value: EthRune) -> Self {
        Into::<EthTerm>::into(value).into()
    }
}

impl From<EthRitchie> for FlyTerm {
    fn from(value: EthRitchie) -> Self {
        Into::<EthTerm>::into(value).into()
    }
}

impl From<SolTerm> for FlyTerm {
    #[inline(always)]
    fn from(term: SolTerm) -> Self {
        Self {
            place: None,
            base: term.into(),
        }
    }
}

impl From<HolTerm> for FlyTerm {
    #[inline(always)]
    fn from(term: HolTerm) -> Self {
        Self {
            place: None,
            base: term.into(),
        }
    }
}

#[test]
fn term_to_fluffy_term_works() {
    fn t(a: impl Copy + Into<EthTerm> + Into<FlyTerm>) {
        let term: EthTerm = a.into();
        let fluffy_term: FlyTerm = a.into();
        let term_to_fluffy_term: FlyTerm = term.into();
        assert_eq!(fluffy_term, term_to_fluffy_term)
    }
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let term_menu = db.ethereal_term_menu(toolchain);
    t(Literal::I8(1))
}

impl FlyTerm {
    pub fn place(self) -> Option<FlyPlace> {
        self.place
    }

    pub fn base_resolved(self, engine: &impl FlyTermEngine) -> FlyTermBase {
        self.base_resolved_inner(engine.fluffy_terms())
    }

    pub fn base_resolved_inner(
        self,
        fluffy_terms: &impl std::borrow::Borrow<HolTerms>,
    ) -> FlyTermBase {
        match self.base {
            FlyTermBase::Eth(_) | FlyTermBase::Sol(_) | FlyTermBase::Place => self.base,
            FlyTermBase::Hol(term) => match term.resolve_progress(fluffy_terms.borrow()) {
                TermResolveProgress::UnresolvedHol => self.base,
                TermResolveProgress::ResolvedEth(term) => term.into(),
                TermResolveProgress::ResolvedSol(term) => term.into(),
                TermResolveProgress::Err => todo!(),
            },
        }
    }

    #[inline(never)]
    pub fn show(self, db: &::salsa::Db, terms: &FlyTerms) -> String {
        self.data_inner(db, terms).show(db, terms)
    }

    pub(crate) fn base(&self) -> FlyTermBase {
        self.base
    }
}
