use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpectAnyOriginal;

impl ExpectLocalTerm for ExpectAnyOriginal {
    type Outcome = ExpectInsSortOutcome;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }

    #[inline(always)]
    fn final_destination(
        &self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        FinalDestination::AnyOriginal
    }
}
