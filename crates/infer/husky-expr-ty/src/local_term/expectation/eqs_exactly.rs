use super::*;

/// expect term to be equal to `Type` i.e. `Sort 1`
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpectSubtype {
    destination: LocalTerm,
}

impl ExpectSubtype {
    pub(crate) fn new(destination: LocalTerm) -> Self {
        Self { destination }
    }
}

impl ExpectLocalTerm for ExpectSubtype {
    type Outcome = ExpectSubtypeOutcome;

    fn retrieve_outcome(outcome: &LocalTermExpectationOutcome) -> &Self::Outcome {
        match outcome {
            LocalTermExpectationOutcome::EqsExactly(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination(
        &self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        self.destination.final_destination(db, unresolved_terms)
    }

    fn destination(&self) -> Option<LocalTerm> {
        Some(self.destination)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExpectSubtypeOutcome {
    destination: LocalTerm,
}

impl ExpectSubtypeOutcome {
    pub(crate) fn resolved(&self) -> Option<Term> {
        todo!()
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn resolve_eqs_exactly(
        &self,
        expectee: LocalTerm,
        expectation: &ExpectSubtype,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        match expectee {
            LocalTerm::Resolved(expectee) => {
                self.eqs_exactly_res_to(expectee, expectation.destination)
            }
            LocalTerm::Unresolved(_) => todo!(),
        }
    }

    #[inline(always)]
    fn eqs_exactly_res_to(
        &self,
        expectee: Term,
        destination: LocalTerm,
    ) -> Option<LocalTermExpectationEffect> {
        match destination {
            LocalTerm::Resolved(destination) => {
                Some(self.eqs_exactly_res_to_res(expectee, destination))
            }
            LocalTerm::Unresolved(_) => todo!(),
        }
    }

    #[inline(always)]
    fn eqs_exactly_res_to_res(
        &self,
        expectee: Term,
        destination: Term,
    ) -> LocalTermExpectationEffect {
        match expectee == destination {
            true => LocalTermExpectationEffect {
                result: Ok(LocalTermExpectationOutcome::EqsExactly(
                    ExpectSubtypeOutcome {
                        destination: destination.into(),
                    },
                )),
                actions: smallvec![],
            },
            false => todo!(),
        }
    }

    #[inline(always)]
    pub(crate) fn expect_eqs_ty0(&self) -> ExpectSubtype {
        ExpectSubtype {
            destination: self.term_menu().ty0().into(),
        }
    }
}
