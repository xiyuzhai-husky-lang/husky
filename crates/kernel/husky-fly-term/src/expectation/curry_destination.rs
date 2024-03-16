use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectCurryDestination {
    curry_destination: FlyTerm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectCurryDestinationOutcome;

impl ExpectCurryDestination {
    pub fn new(curry_destination: FlyTerm) -> Self {
        Self { curry_destination }
    }
}

impl ExpectFlyTerm for ExpectCurryDestination {
    type Outcome = ExpectCurryDestinationOutcome;

    #[inline(always)]
    fn retrieve_outcome(outcome: &ExpectationOutcome) -> &Self::Outcome {
        &ExpectCurryDestinationOutcome
    }

    #[inline(always)]
    fn final_destination_inner(&self, db: &::salsa::Db, fly_terms: &FlyTerms) -> FinalDestination {
        self.curry_destination
            .final_destination_inner(db, fly_terms)
    }

    #[inline(always)]
    fn destination(&self) -> FlyTermDestination {
        // todo: refine
        FlyTermDestination::AnyOriginal
    }

    fn resolve(
        &self,
        db: &::salsa::Db,
        terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        AltNone
    }
}
