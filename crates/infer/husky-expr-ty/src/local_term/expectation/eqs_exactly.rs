use super::*;

/// expect term to be equal to `Type` i.e. `Sort 1`
#[derive(Debug, Clone)]
pub(crate) struct ExpectEqsExactly {
    destination: LocalTerm,
}

impl ExpectLocalTerm for ExpectEqsExactly {
    type Result = ExpectEqsExactlyResult;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExpectEqsExactlyResult {
    local_term: LocalTerm,
}
impl ExpectEqsExactlyResult {
    pub(crate) fn resolved(&self) -> Option<ReducedTerm> {
        todo!()
    }
}

impl From<ExpectEqsExactlyResult> for LocalTermExpectationResult {
    fn from(value: ExpectEqsExactlyResult) -> Self {
        LocalTermExpectationResult::OkEqsExactly(value)
    }
}

impl From<ExpectEqsExactly> for LocalTermExpectation {
    fn from(value: ExpectEqsExactly) -> Self {
        LocalTermExpectation::EqsExactly {
            destination: value.destination,
        }
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn resolve_eqs_exactly(
        &self,
        expectee: LocalTerm,
        destination: LocalTerm,
    ) -> Option<LocalTermExpectationResultM> {
        match expectee {
            LocalTerm::Resolved(expectee) => self.eqs_exactly_res_to(expectee, destination),
            LocalTerm::Unresolved(_) => todo!(),
        }
    }

    #[inline(always)]
    fn eqs_exactly_res_to(
        &self,
        expectee: ReducedTerm,
        destination: LocalTerm,
    ) -> Option<LocalTermExpectationResultM> {
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
    ) -> LocalTermExpectationResultM {
        match expectee == destination {
            true => LocalTermExpectationResultM {
                result: LocalTermExpectationResult::OkEqsExactly(ExpectEqsExactlyResult {
                    local_term: expectee.into(),
                }),
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
