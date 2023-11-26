use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectAnyDerived;

impl ExpectFluffyTerm for ExpectAnyDerived {
    type Outcome = ();

    #[inline(always)]
    fn retrieve_outcome(_outcome: &FluffyTermExpectationOutcome) -> &Self::Outcome {
        &()
    }

    #[inline(always)]
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FluffyTerms) -> FinalDestination {
        FinalDestination::AnyDerived
    }

    #[inline(always)]
    fn destination(&self) -> Option<FluffyTerm> {
        None
    }

    fn resolve(
        &self,
        db: &::salsa::Db,
        fluffy_terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FluffyTermEffect> {
        AltNone
    }
}
