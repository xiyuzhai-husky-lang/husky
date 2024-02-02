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
use husky_term_prelude::literal::TermLiteral;

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
    Ethereal(EthTerm),
    Solid(SolidTerm),
    Hollow(HollowTerm),
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

impl From<TermLiteral> for FlyTerm {
    fn from(value: TermLiteral) -> Self {
        Into::<EthTerm>::into(value).into()
    }
}

impl From<ItemPathTerm> for FlyTerm {
    fn from(value: ItemPathTerm) -> Self {
        Into::<EthTerm>::into(value).into()
    }
}

impl From<ApplicationEthTerm> for FlyTerm {
    fn from(value: ApplicationEthTerm) -> Self {
        Into::<EthTerm>::into(value).into()
    }
}

impl From<CurryEthTerm> for FlyTerm {
    fn from(value: CurryEthTerm) -> Self {
        Into::<EthTerm>::into(value).into()
    }
}

impl From<CategoryTerm> for FlyTerm {
    fn from(value: CategoryTerm) -> Self {
        Into::<EthTerm>::into(value).into()
    }
}

impl From<SymbolEthTerm> for FlyTerm {
    fn from(value: SymbolEthTerm) -> Self {
        Into::<EthTerm>::into(value).into()
    }
}

impl From<RuneEthTerm> for FlyTerm {
    fn from(value: RuneEthTerm) -> Self {
        Into::<EthTerm>::into(value).into()
    }
}

impl From<RitchieEthTerm> for FlyTerm {
    fn from(value: RitchieEthTerm) -> Self {
        Into::<EthTerm>::into(value).into()
    }
}

impl From<SolidTerm> for FlyTerm {
    #[inline(always)]
    fn from(term: SolidTerm) -> Self {
        Self {
            place: None,
            base: term.into(),
        }
    }
}

impl From<HollowTerm> for FlyTerm {
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
    fn t(a: impl Copy + Into<EthTerm> + Into<FlyTerm>) {
        let term: EthTerm = a.into();
        let fluffy_term: FlyTerm = a.into();
        let term_to_fluffy_term: FlyTerm = term.into();
        assert_eq!(fluffy_term, term_to_fluffy_term)
    }
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let term_menu = db.ethereal_term_menu(toolchain);
    t(TermLiteral::I8(1))
}

impl FlyTerm {
    #[deprecated(note = "should return place or panic")]
    pub fn place(self) -> Option<FlyPlace> {
        self.place
    }

    pub fn base_resolved(self, engine: &impl FlyTermEngine) -> FlyTermBase {
        self.base_resolved_inner(engine.fluffy_terms())
    }

    pub fn base_resolved_inner(
        self,
        fluffy_terms: &impl std::borrow::Borrow<HollowTerms>,
    ) -> FlyTermBase {
        match self.base {
            FlyTermBase::Ethereal(_) | FlyTermBase::Solid(_) => self.base,
            FlyTermBase::Hollow(term) => match term.resolve_progress(fluffy_terms.borrow()) {
                TermResolveProgress::UnresolvedHollow => self.base,
                TermResolveProgress::ResolvedEthereal(term) => term.into(),
                TermResolveProgress::ResolvedSolid(term) => term.into(),
                TermResolveProgress::Err => todo!(),
            },
            FlyTermBase::Place => self.base,
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
