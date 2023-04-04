use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpectAnyDerived;

impl ExpectLocalTerm for ExpectAnyDerived {
    type Outcome = ();

    fn retrieve_outcome(_outcome: &LocalTermExpectationOutcome) -> &Self::Outcome {
        &()
    }

    #[inline(always)]
    fn final_destination(
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
