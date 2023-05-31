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
    fn final_destination_inner(
        &self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> FinalDestination {
        FinalDestination::AnyDerived
    }

    #[inline(always)]
    fn destination(&self) -> Option<FluffyTerm> {
        None
    }
}
