use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectAnyOriginal;

impl ExpectLocalTerm for ExpectAnyOriginal {
    type Outcome = LocalTerm;

    fn retrieve_outcome(_outcome: &LocalTermExpectationOutcome) -> &Self::Outcome {
        todo!()
    }

    #[inline(always)]
    fn final_destination_inner(
        &self,
        db: &dyn TermDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        FinalDestination::AnyOriginal
    }

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}
