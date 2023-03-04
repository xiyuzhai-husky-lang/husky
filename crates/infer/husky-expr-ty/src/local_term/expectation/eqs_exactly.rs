use super::*;

/// expect term to be equal to `Type` i.e. `Sort 1`
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpectEqsExactly {
    destination: LocalTerm,
}

impl ProvideEntityPathTypeExpectation for ExpectEqsExactly {
    fn entity_path_ty_expectation(
        &self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> EntityPathTypeExpectation {
        self.destination
            .entity_path_ty_expectation(db, unresolved_terms)
    }
}

impl ExpectLocalTerm for ExpectEqsExactly {
    type Outcome = ExpectEqsExactlyOutcome;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }

    #[inline(always)]
    fn final_destination(
        &self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExpectEqsExactlyOutcome {
    destination: LocalTerm,
}

impl ExpectLocalTermOutcome for ExpectEqsExactlyOutcome {
    fn destination(&self) -> LocalTerm {
        self.destination
    }

    fn downcast_ref(resolved_ok: &LocalTermExpectationOutcome) -> &Self {
        match resolved_ok {
            LocalTermExpectationOutcome::EqsExactly(resolved_ok) => resolved_ok,
            _ => unreachable!(),
        }
    }
}

impl ExpectEqsExactlyOutcome {
    pub(crate) fn resolved(&self) -> Option<ReducedTerm> {
        todo!()
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn resolve_eqs_exactly(
        &self,
        expectee: LocalTerm,
        expectation: &ExpectEqsExactly,
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
        expectee: ReducedTerm,
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
        expectee: ReducedTerm,
        destination: ReducedTerm,
    ) -> LocalTermExpectationEffect {
        match expectee == destination {
            true => LocalTermExpectationEffect {
                result: Ok(LocalTermExpectationOutcome::EqsExactly(
                    ExpectEqsExactlyOutcome {
                        destination: destination.into(),
                    },
                )),
                actions: vec![],
            },
            false => todo!(),
        }
    }

    #[inline(always)]
    pub(crate) fn expect_eqs_exactly_ty(&self) -> ExpectEqsExactly {
        ExpectEqsExactly {
            destination: self.reduced_term_menu().ty0().into(),
        }
    }
}
