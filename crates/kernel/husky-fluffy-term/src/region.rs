mod expectations;
mod terms;

pub use self::expectations::*;
pub use self::terms::*;

use crate::*;

// `Default` is not implemented because we might need to initialize `terms` from the parent
#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb)]
pub struct FluffyTermRegion {
    pub(crate) terms: FluffyTerms,
    pub(crate) expectations: Expectations,
}

impl std::borrow::Borrow<HollowTerms> for FluffyTermRegion {
    fn borrow(&self) -> &HollowTerms {
        self.terms.borrow()
    }
}

impl FluffyTermRegion {
    pub fn new(parent: Option<&Self>) -> Self {
        Self {
            terms: FluffyTerms::new(parent.map(|parent| &parent.terms)),
            expectations: Default::default(),
        }
    }

    pub fn terms(&self) -> &FluffyTerms {
        &self.terms
    }

    pub(crate) fn terms_mut(&mut self) -> &mut FluffyTerms {
        &mut self.terms
    }

    pub fn expectations(&self) -> &Expectations {
        &self.expectations
    }

    pub fn solid_terms(&self) -> &SolidTerms {
        self.terms.solid_terms()
    }

    pub(crate) fn solid_terms_mut(&mut self) -> &mut SolidTerms {
        self.terms.solid_terms_mut()
    }

    pub fn hollow_terms(&self) -> &HollowTerms {
        self.terms.hollow_terms()
    }

    pub(crate) fn hollow_terms_mut(&mut self) -> &mut HollowTerms {
        self.terms.hollow_terms_mut()
    }

    pub fn finalize_unresolved_term_table(
        &mut self,
        db: &dyn FluffyTermDb,
        term_menu: &EtherealTermMenu,
    ) {
        self.resolve_as_much_as_possible(db);
        self.terms.fill_all_holes(db, term_menu)
    }
}

impl std::ops::Index<ExpectationIdx> for FluffyTermRegion {
    type Output = ExpectationEntry;

    fn index(&self, index: ExpectationIdx) -> &Self::Output {
        &self.expectations[index]
    }
}
