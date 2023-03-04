use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpectAnyOriginal;

impl const ProvideEntityPathTypeExpectation for ExpectAnyOriginal {
    fn entity_path_ty_expectation(
        &self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> EntityPathTypeExpectation {
        EntityPathTypeExpectation::Any
    }
}

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
        FinalDestination::NoneOriginal
    }
}
