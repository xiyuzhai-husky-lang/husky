use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectAnyOriginal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectAnyOriginalOutcome;

impl ExpectFlyTerm for ExpectAnyOriginal {
    type Outcome = ExpectAnyOriginalOutcome;

    fn initial_resolve_progress() -> ExpectationProgress {
        ExpectationProgress::Resolved(Ok(ExpectAnyOriginalOutcome.into()))
    }

    #[inline(always)]
    fn retrieve_outcome(_outcome: &ExpectationOutcome) -> &Self::Outcome {
        &ExpectAnyOriginalOutcome
    }

    #[inline(always)]
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FlyTerms) -> FinalDestination {
        FinalDestination::AnyOriginal
    }

    #[inline(always)]
    fn destination(&self) -> FlyTermDestination {
        FlyTermDestination::AnyOriginal
    }

    fn resolve(
        &self,
        _db: &::salsa::Db,
        _terms: &mut FlyTerms,
        _state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        AltNone
    }
}
