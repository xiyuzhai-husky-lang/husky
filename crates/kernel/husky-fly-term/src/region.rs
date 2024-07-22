mod expectations;
mod terms;

pub use self::expectations::*;
pub use self::terms::*;

use crate::*;

// `Default` is not implemented because we might need to initialize `terms` from the parent
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct FlyTermRegion {
    pub(crate) terms: FlyTerms,
    pub(crate) expectations: Expectations,
}

impl std::borrow::Borrow<HolTerms> for FlyTermRegion {
    fn borrow(&self) -> &HolTerms {
        self.terms.borrow()
    }
}

impl FlyTermRegion {
    pub fn new(parent: Option<&Self>) -> Self {
        Self {
            terms: FlyTerms::new(parent.map(|parent| &parent.terms)),
            expectations: Default::default(),
        }
    }

    pub fn terms(&self) -> &FlyTerms {
        &self.terms
    }

    pub(crate) fn terms_mut(&mut self) -> &mut FlyTerms {
        &mut self.terms
    }

    pub fn expectations(&self) -> &Expectations {
        &self.expectations
    }

    pub fn sol_terms(&self) -> &SolTerms {
        self.terms.sol_terms()
    }

    pub(crate) fn solid_terms_mut(&mut self) -> &mut SolTerms {
        self.terms.sol_terms_mut()
    }

    pub fn hollow_terms(&self) -> &HolTerms {
        self.terms.hol_terms()
    }

    pub(crate) fn hollow_terms_mut(&mut self) -> &mut HolTerms {
        self.terms.hol_terms_mut()
    }

    pub fn finalize_unresolved_term_table(&mut self, db: &::salsa::Db, term_menu: &EthTermMenu) {
        self.resolve_as_much_as_possible(db);
        self.terms.fill_all_holes(db, term_menu)
    }
}

impl std::ops::Index<FlyTermExpectationIdx> for FlyTermRegion {
    type Output = FlyTermExpectationEntry;

    fn index(&self, index: FlyTermExpectationIdx) -> &Self::Output {
        &self.expectations[index]
    }
}
