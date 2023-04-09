mod hollow;
mod sealed;

pub(crate) use self::hollow::*;
pub(crate) use self::sealed::*;

use super::*;

#[derive(Debug, Default,PartialEq, Eq)]
pub struct FluffyTerms {
    hollow_terms: HollowTerms,
    sealed_terms: SolidTerms,
}

impl FluffyTerms {
    pub(crate) fn new_hole_from_parameter_symbol(
        &mut self,
        db: &dyn FluffyTermDb,
        src: HollowTermSource,
        parameter_symbol: FluffyTerm,
    ) -> HollowTerm {
        todo!()
    }
}
