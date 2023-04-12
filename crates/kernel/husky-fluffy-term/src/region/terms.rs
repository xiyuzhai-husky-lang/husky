mod hollow;
mod solid;

pub use self::hollow::*;
pub use self::solid::*;

use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct FluffyTerms {
    hollow_terms: HollowTerms,
    solid_terms: SolidTerms,
}

impl FluffyTerms {
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
