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

/// final destination of `A1 -> ... -> An` is equal to that of `An`
///
/// final destination of `A1 ... An` is equal to that of `A1`
///
/// final destination of `Sort` is `FinalDestination::Sort`
///
/// final destination of a type path `A` is `FinalDestination::TypePath(A)`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub enum FinalDestination {
    Sort,
    TypeOntology,
    AnyOriginal,
    AnyDerived,
}
