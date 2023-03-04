use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpectAnyDerived;

impl const ProvideEntityPathTypeExpectation for ExpectAnyDerived {
    fn entity_path_ty_expectation(
        &self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> EntityPathTypeExpectation {
        EntityPathTypeExpectation::Any
    }
}

impl ExpectLocalTerm for ExpectAnyDerived {
    type Outcome = ExpectInsSortOutcome;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }

    #[inline(always)]
    fn final_destination(
        &self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> ExprTypeResult<LocalTerm> {
        Err(DerivedExprTypeError::FinalDestination.into())
    }
}
