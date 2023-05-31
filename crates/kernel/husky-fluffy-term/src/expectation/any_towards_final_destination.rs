use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectAnyTowardsFinalDestination {
    final_destination: FinalDestination,
}

impl ExpectAnyTowardsFinalDestination {
    pub fn new(final_destination: FinalDestination) -> Self {
        Self { final_destination }
    }
}

impl ExpectFluffyTerm for ExpectAnyTowardsFinalDestination {
    type Outcome = ();

    #[inline(always)]
    fn retrieve_outcome(outcome: &FluffyTermExpectationOutcome) -> &Self::Outcome {
        &()
    }

    #[inline(always)]
    fn final_destination_inner(
        &self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> FinalDestination {
        self.final_destination
    }

    #[inline(always)]
    fn destination(&self) -> Option<FluffyTerm> {
        None
    }
}
