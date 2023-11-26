use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectFinalDestination {
    final_destination: FinalDestination,
}

impl ExpectFinalDestination {
    pub fn new(final_destination: FinalDestination) -> Self {
        Self { final_destination }
    }
}

impl ExpectFluffyTerm for ExpectFinalDestination {
    type Outcome = ();

    #[inline(always)]
    fn retrieve_outcome(outcome: &FluffyTermExpectationOutcome) -> &Self::Outcome {
        &()
    }

    #[inline(always)]
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FluffyTerms) -> FinalDestination {
        self.final_destination
    }

    #[inline(always)]
    fn destination(&self) -> Option<FluffyTerm> {
        None
    }

    fn resolve(
        &self,
        _db: &::salsa::Db,
        _terms: &mut FluffyTerms,
        _state: &mut ExpectationState,
    ) -> AltOption<FluffyTermEffect> {
        // ad hoc
        AltNone
    }
}
