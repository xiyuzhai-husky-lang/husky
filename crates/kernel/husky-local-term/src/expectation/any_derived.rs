use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectAnyDerived;

impl ExpectLocalTerm for ExpectAnyDerived {
    type Outcome = ();

    fn retrieve_outcome(_outcome: &LocalTermExpectationOutcome) -> &Self::Outcome {
        &()
    }

    #[inline(always)]
    fn final_destination_inner(
        &self,
        db: &dyn TermDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        FinalDestination::AnyDerived
    }

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}
