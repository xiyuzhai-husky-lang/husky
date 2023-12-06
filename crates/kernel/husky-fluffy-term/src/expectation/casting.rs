use super::*;

/// expect a type that is explicitly convertible to dst
#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
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
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub struct ExpectExplicitlyConvertibleOutcome {
    destination: FluffyTerm,
}

impl ExpectFluffyTerm for ExpectCasting {
    type Outcome = ExpectExplicitlyConvertibleOutcome;

    #[inline(always)]
    fn retrieve_outcome(outcome: &ExpectationOutcome) -> &Self::Outcome {
        match outcome {
            ExpectationOutcome::ExplicitlyConvertible(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FluffyTerms) -> FinalDestination {
        todo!()
    }

    #[inline(always)]
    fn destination(&self) -> Option<FluffyTerm> {
        Some(self.destination)
    }

    fn resolve(
        &self,
        db: &::salsa::Db,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FluffyTermEffect> {
        // todo
        AltOption::AltNone
    }
}
