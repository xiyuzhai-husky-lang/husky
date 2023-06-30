use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectAnyOriginal;

impl ExpectFluffyTerm for ExpectAnyOriginal {
    type Outcome = FluffyTerm;

    #[inline(always)]
    fn retrieve_outcome(_outcome: &FluffyTermExpectationOutcome) -> &Self::Outcome {
        todo!()
    }

    #[inline(always)]
    fn final_destination_inner(
        &self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> FinalDestination {
        FinalDestination::AnyOriginal
    }

    #[inline(always)]
    fn destination(&self) -> Option<FluffyTerm> {
        None
    }

    fn resolve(
        &self,
        db: &dyn FluffyTermDb,
        entry: &mut ExpectationMeta,
        fluffy_terms: &mut FluffyTerms,
    ) -> Option<ExpectationEffect> {
        None
    }
}
