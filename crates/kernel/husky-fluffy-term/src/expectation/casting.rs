use super::*;

/// expect a type that is explicitly convertible to dst
#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ExpectCasting {
    pub(crate) destination: FluffyTerm,
}

impl ExpectCasting {
    pub fn new(destination: FluffyTerm) -> Self {
        Self { destination }
    }

    pub(crate) fn try_substitute_unresolved_fluffy_term<'a>(
        &self,
        fluffy_terms: &'a FluffyTerms,
    ) -> Result<Option<Expectation>, &'a HollowTermResolveError> {
        todo!()
        // match fluffy_terms.try_reduce_fluffy_term(self.destination)? {
        //     Some(destination) => Ok(Some(ExpectExplicitlyConvertible { destination }.into())),
        //     None => Ok(None),
        // }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ExpectExplicitlyConvertibleOutcome {
    destination: FluffyTerm,
}

impl ExpectFluffyTerm for ExpectCasting {
    type Outcome = ExpectExplicitlyConvertibleOutcome;

    #[inline(always)]
    fn retrieve_outcome(outcome: &FluffyTermExpectationOutcome) -> &Self::Outcome {
        match outcome {
            FluffyTermExpectationOutcome::ExplicitlyConvertible(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(
        &self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> FinalDestination {
        todo!()
    }

    #[inline(always)]
    fn destination(&self) -> Option<FluffyTerm> {
        Some(self.destination)
    }

    fn resolve(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> Option<ExpectationEffect> {
        // todo
        None
    }
}
