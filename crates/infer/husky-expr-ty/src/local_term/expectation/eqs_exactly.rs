use super::*;

/// expect term to be equal to `Type` i.e. `Sort 1`
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpectSubtype {
    // todo: add variance,
    expected: LocalTerm,
}

impl ExpectSubtype {
    pub(crate) fn new(destination: LocalTerm) -> Self {
        Self {
            expected: destination,
        }
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
        self.expected.final_destination(db, unresolved_terms)
    }

    fn destination(&self) -> Option<LocalTerm> {
        Some(self.expected)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExpectSubtypeOutcome {
    // todo: change this to option lifetime subtype constraint
}

impl ExpectSubtypeOutcome {
    pub(crate) fn resolved(&self) -> Option<Term> {
        todo!()
    }
}

impl ExpectSubtype {
    pub(super) fn resolve(
        &self,
        db: &dyn ExprTypeDb,
        src_expr_idx: ExprIdx,
        expectee: LocalTerm,
        unresolved_terms: &UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        if expectee == self.expected {
            return Some(LocalTermExpectationEffect {
                result: Ok(ExpectSubtypeOutcome {}.into()),
                actions: smallvec![],
            });
        }
        match self.expected.pattern(db, unresolved_terms) {
            LocalTermPattern::Literal(_) => todo!(),
            LocalTermPattern::TypeOntology {
                path: expected_path,
                argument_tys,
                ..
            } => match expectee.pattern(db, unresolved_terms) {
                LocalTermPattern::TypeOntology {
                    path: expectee_path,
                    argument_tys,
                    ..
                } => {
                    if expected_path == expectee_path {
                        todo!()
                    } else {
                        Some(LocalTermExpectationEffect {
                            result: Err(OriginalLocalTermExpectationError::TypePathMismatch {
                                expected_path,
                                expectee_path,
                            }
                            .into()),
                            actions: smallvec![],
                        })
                    }
                }
                LocalTermPattern::ImplicitSymbol(_, _) => todo!(),
                _ => Some(LocalTermExpectationEffect {
                    result: Err(todo!()),
                    actions: smallvec![],
                }),
            },
            LocalTermPattern::Curry {
                curry_kind,
                variance,
                parameter_symbol,
                parameter_ty,
                return_ty,
            } => todo!(),
            LocalTermPattern::ImplicitSymbol(_, _) => todo!(),
            LocalTermPattern::Category(_) => todo!(),
            LocalTermPattern::Ritchie {
                ritchie_kind,
                parameter_liasoned_tys,
                return_ty,
            } => todo!(),
        }
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
                self.eqs_exactly_res_to(expectee, expectation.expected)
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
                    ExpectSubtypeOutcome {},
                )),
                actions: smallvec![],
            },
            false => todo!(),
        }
    }

    #[inline(always)]
    pub(crate) fn expect_eqs_ty0(&self) -> ExpectSubtype {
        ExpectSubtype {
            expected: self.term_menu().ty0().into(),
        }
    }
}
