use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectAnyOriginal;

impl ExpectFluffyTerm for ExpectAnyOriginal {
    type Outcome = ();

    #[inline(always)]
    fn retrieve_outcome(_outcome: &FluffyTermExpectationOutcome) -> &Self::Outcome {
        &()
    }

    #[inline(always)]
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FluffyTerms) -> FinalDestination {
        FinalDestination::AnyOriginal
    }

    #[inline(always)]
    fn destination(&self) -> Option<FluffyTerm> {
        None
    }

    fn resolve(
        &self,
        _db: &::salsa::Db,
        _terms: &mut FluffyTerms,
        _state: &mut ExpectationState,
    ) -> AltOption<FluffyTermEffect> {
        AltNone
    }
}
