mod expectations;
mod terms;

pub use self::expectations::*;
pub use self::terms::*;

use crate::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct FluffyTermRegion {
    terms: FluffyTerms,
    expectations: FluffyTermExpectations,
}

impl FluffyTermRegion {
    pub fn terms(&self) -> &FluffyTerms {
        &self.terms
    }

    pub fn expectations(&self) -> &FluffyTermExpectations {
        &self.expectations
    }

    pub fn hollow_terms(&self) -> &HollowTerms {
        self.terms.hollow_terms()
    }

    pub fn solid_terms(&self) -> &SolidTerms {
        self.terms.solid_terms()
    }

    pub fn hollow_terms_mut(&mut self) -> &HollowTerms {
        self.terms.hollow_terms_mut()
    }

    pub fn solid_terms_mut(&mut self) -> &mut SolidTerms {
        self.terms.solid_terms_mut()
    }

    pub(crate) fn expectations_mut(&mut self) -> &mut FluffyTermExpectations {
        &mut self.expectations
    }

    pub fn finalize_unresolved_term_table(&mut self, db: &dyn FluffyTermDb) {
        self.resolve_as_much_as_possible(db, FluffyTermResolveLevel::Strong);
        // ad hoc
        // todo!()
    }
}
