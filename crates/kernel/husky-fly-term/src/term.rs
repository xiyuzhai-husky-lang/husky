pub mod application;
pub mod curry;
pub mod hole;
pub mod hvar;
pub mod quary;
pub mod ritchie;
pub mod symbol_ty;
pub mod ty_as_trai_item;
mod utils;

pub use self::hole::*;
pub use self::hvar::*;
pub use self::ritchie::*;
pub use self::symbol_ty::*;
use crate::quary::FlyQuary;

use crate::*;
use husky_eth_term::term::{
    application::EthApplication, curry::EthCurry, lambda_variable::EthLambdaVariable,
    ritchie::EthRitchie, symbolic_variable::EthSymbolicVariable,
};
use husky_term_prelude::literal::Literal;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FlyTerm {
    place: Option<FlyQuary>,
    base: FlyTermBase,
}

impl FlyTerm {
    pub(crate) fn new_ethereal(place: FlyQuary, ethereal_term: EthTerm) -> Self {
        Self {
            place: Some(place),
            base: ethereal_term.into(),
        }
    }

    pub fn with_quary(self, place: FlyQuary) -> Self {
        Self {
            place: Some(place),
            base: self.base,
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum FlyTermBase {
    Eth(EthTerm),
    Sol(SolTerm),
    Hol(HolTerm),
    Place,
}

impl From<FlyQuary> for FlyTerm {
    fn from(place: FlyQuary) -> Self {
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

impl From<Sort> for FlyTerm {
    fn from(value: Sort) -> Self {
        Into::<EthTerm>::into(value).into()
    }
}

impl From<EthSymbolicVariable> for FlyTerm {
    fn from(value: EthSymbolicVariable) -> Self {
        Into::<EthTerm>::into(value).into()
    }
}

impl From<EthLambdaVariable> for FlyTerm {
    fn from(value: EthLambdaVariable) -> Self {
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
fn term_to_fly_term_works() {
    fn t(a: impl Copy + Into<EthTerm> + Into<FlyTerm>) {
        let term: EthTerm = a.into();
        let fly_term: FlyTerm = a.into();
        let term_to_fly_term: FlyTerm = term.into();
        assert_eq!(fly_term, term_to_fly_term)
    }
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let term_menu = db.ethereal_term_menu(toolchain);
    t(Literal::I8(1))
}

impl FlyTerm {
    pub fn quary(self) -> Option<FlyQuary> {
        self.place
    }

    pub fn base_resolved(self, engine: &impl FlyTermEngine) -> FlyTermBase {
        self.base_resolved_inner(engine.fly_terms())
    }

    pub fn base_resolved_inner(
        self,
        fly_terms: &impl std::borrow::Borrow<HolTerms>,
    ) -> FlyTermBase {
        match self.base {
            FlyTermBase::Eth(_) | FlyTermBase::Sol(_) | FlyTermBase::Place => self.base,
            FlyTermBase::Hol(term) => match term.resolve_progress(fly_terms.borrow()) {
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
