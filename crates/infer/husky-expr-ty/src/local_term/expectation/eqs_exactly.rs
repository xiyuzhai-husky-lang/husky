use super::*;

/// expect term to be equal to `Type` i.e. `Sort 1`
#[derive(Debug, Clone)]
pub(crate) struct ExpectEqsExactly {
    destination: LocalTerm,
}

impl ExpectLocalTerm for ExpectEqsExactly {
    type ResolvedOk = ExpectEqsExactlyResolvedOk;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExpectEqsExactlyResolvedOk {
    destination: LocalTerm,
}

impl ExpectLocalTermResolvedOk for ExpectEqsExactlyResolvedOk {
    fn destination(&self) -> LocalTerm {
        self.destination
    }

    fn downcast(resolved_ok: &LocalTermExpectationResolvedOk) -> Self {
        todo!()
    }
}

impl ExpectEqsExactlyResolvedOk {
    pub(crate) fn resolved(&self) -> Option<ReducedTerm> {
        todo!()
    }
}

impl From<ExpectEqsExactlyResolvedOk> for LocalTermExpectationResolvedOk {
    fn from(value: ExpectEqsExactlyResolvedOk) -> Self {
        LocalTermExpectationResolvedOk::OkEqsExactly(value)
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
    ) -> Option<LocalTermExpectationResolvedOkM> {
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
    ) -> Option<LocalTermExpectationResolvedOkM> {
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
    ) -> LocalTermExpectationResolvedOkM {
        match expectee == destination {
            true => LocalTermExpectationResolvedOkM {
                result: Ok(LocalTermExpectationResolvedOk::OkEqsExactly(
                    ExpectEqsExactlyResolvedOk {
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
