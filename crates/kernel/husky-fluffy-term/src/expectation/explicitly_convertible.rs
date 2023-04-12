use super::*;

/// expect a type that is explicitly convertible to dst
#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ExpectExplicitlyConvertible {
    pub(crate) destination: FluffyTerm,
}

impl ExpectExplicitlyConvertible {
    pub fn new(destination: FluffyTerm) -> Self {
        Self { destination }
    }

    pub(crate) fn try_substitute_unresolved_fluffy_term<'a>(
        &self,
        porous_terms: &'a FluffyTerms,
    ) -> Result<Option<ExpectationData>, &'a HollowTermResolveError> {
        todo!()
        // match porous_terms.try_reduce_fluffy_term(self.destination)? {
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

impl ExpectLocalTerm for ExpectExplicitlyConvertible {
    type Outcome = ExpectExplicitlyConvertibleOutcome;

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

    fn destination(&self) -> Option<FluffyTerm> {
        Some(self.destination)
    }
}

impl ExpectExplicitlyConvertible {
    pub(super) fn resolve(
        &self,
        db: &dyn FluffyTermDb,
        porous_terms: &mut FluffyTerms,
        expectee: FluffyTerm,
        level: FluffyTermResolveLevel,
    ) -> Option<FluffyTermExpectationEffect> {
        // todo
        None
    }
}
