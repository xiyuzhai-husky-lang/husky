mod hollow;
mod solid;

pub use self::hollow::*;
pub use self::solid::*;

use super::*;

// `Default` is not implemented because we might need to initialize `solid_terms` from the parent
#[derive(Debug, PartialEq, Eq)]
pub struct FluffyTerms {
    hollow_terms: HollowTerms,
    solid_terms: SolidTerms,
}

impl FluffyTerms {
    pub(crate) fn new(terms: Option<&Self>) -> Self {
        Self {
            // `Default` is derived for `hollow_terms` because we never inherited hollow terms
            hollow_terms: Default::default(),
            solid_terms: SolidTerms::new(terms.map(|terms| &terms.solid_terms)),
        }
    }

    pub(crate) fn new_hole_from_parameter_symbol(
        &mut self,
        db: &dyn FluffyTermDb,
        src: HoleSource,
        parameter_symbol: FluffyTerm,
    ) -> HollowTerm {
        todo!()
    }

    pub fn hollow_terms(&self) -> &HollowTerms {
        &self.hollow_terms
    }

    pub fn solid_terms(&self) -> &SolidTerms {
        &self.solid_terms
    }

    pub fn hollow_terms_mut(&mut self) -> &mut HollowTerms {
        &mut self.hollow_terms
    }

    pub fn solid_terms_mut(&mut self) -> &mut SolidTerms {
        &mut self.solid_terms
    }
}
