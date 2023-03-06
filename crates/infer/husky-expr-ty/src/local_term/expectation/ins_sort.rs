use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTermDb)]
pub(crate) struct ExpectInsSort {
    smallest_universe: TermUniverse,
}

impl ExpectLocalTerm for ExpectInsSort {
    type Outcome = ExpectInsSortOutcome;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }

    #[inline(always)]
    fn final_destination(
        &self,
        db: &dyn ExprTermDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        todo!()
    }
}

impl ExpectInsSort {
    pub(crate) fn new(u: u8) -> Self {
        ExpectInsSort {
            smallest_universe: u.into(),
        }
    }

    pub(crate) fn smallest_universe(&self) -> TermUniverse {
        self.smallest_universe
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = ExprTermDb)]
pub(crate) struct ExpectInsSortOutcome {
    destination: LocalTerm,
}

impl ExpectInsSortOutcome {
    pub(crate) fn resolved(&self) -> Option<Term> {
        todo!()
    }
}

impl ExpectLocalTermOutcome for ExpectInsSortOutcome {
    fn destination(&self) -> LocalTerm {
        self.destination
    }

    fn downcast_ref(resolved_ok: &LocalTermExpectationOutcome) -> &Self {
        match resolved_ok {
            LocalTermExpectationOutcome::InsSort(resolved_ok) => resolved_ok,
            _ => unreachable!(),
        }
    }
}

impl<'a> ExprTypeEngine<'a> {
    /// try to tell if a term is an instance of `Type u` for some universe u
    pub(super) fn resolve_ins_sort_expectation(
        &self,
        expectee: LocalTerm,
        expectation: &ExpectInsSort,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        match expectee {
            LocalTerm::Resolved(resolved_expectee) => {
                let expectee_ty = term_ty(
                    self.db(),
                    todo!(),
                    resolved_expectee,
                    self.toolchain(),
                    self.term_menu(),
                );
                Some(match expectee_ty {
                    Ok(expectee_ty) => match expectee_ty {
                        Term::Category(cat) => {
                            match cat.universe() >= expectation.smallest_universe {
                                true => LocalTermExpectationEffect {
                                    result: Ok(LocalTermExpectationOutcome::InsSort(
                                        ExpectInsSortOutcome {
                                            destination: expectee,
                                        },
                                    )),
                                    actions: vec![],
                                },
                                false => LocalTermExpectationEffect {
                                    result: Err(todo!()),
                                    actions: vec![],
                                },
                            }
                        }
                        _ => LocalTermExpectationEffect {
                            result: Err(todo!()),
                            actions: vec![],
                        },
                    },
                    Err(error) => LocalTermExpectationEffect {
                        result: Err(match error {
                            TypeError::Original(_) => {
                                OriginalLocalTermExpectationError::TermTypeError {
                                    term: resolved_expectee,
                                    error,
                                }
                                .into()
                            }
                            TypeError::Derived(_) => {
                                DerivedLocalTermExpectationError::TermTypeError {
                                    term: resolved_expectee,
                                    error,
                                }
                                .into()
                            }
                        }),
                        actions: vec![],
                    },
                })
            }
            LocalTerm::Unresolved(_) => None,
        }
    }
}
