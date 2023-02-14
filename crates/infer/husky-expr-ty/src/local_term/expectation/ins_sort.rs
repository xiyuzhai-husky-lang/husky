use super::*;

#[derive(Debug, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectInsSort {
    smallest_universe: TermUniverse,
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

impl Default for ExpectInsSort {
    fn default() -> Self {
        Self {
            smallest_universe: 0.into(),
        }
    }
}

impl ExpectLocalTerm for ExpectInsSort {
    type ResolvedOk = ExpectInsSortResolvedOk;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectInsSortResolvedOk {
    destination: LocalTerm,
}

impl ExpectInsSortResolvedOk {
    pub(crate) fn resolved(&self) -> Option<ReducedTerm> {
        todo!()
    }
}

impl ExpectLocalTermResolvedOk for ExpectInsSortResolvedOk {
    fn destination(&self) -> LocalTerm {
        self.destination
    }

    fn downcast_ref(resolved_ok: &LocalTermExpectationResolvedOk) -> &Self {
        match resolved_ok {
            LocalTermExpectationResolvedOk::InsSort(resolved_ok) => resolved_ok,
            _ => unreachable!(),
        }
    }
}

impl From<ExpectInsSort> for LocalTermExpectation {
    fn from(value: ExpectInsSort) -> Self {
        LocalTermExpectation::InsSort {
            smallest_universe: value.smallest_universe,
        }
    }
}

impl From<ExpectInsSortResolvedOk> for LocalTermExpectationResolvedOk {
    fn from(value: ExpectInsSortResolvedOk) -> Self {
        LocalTermExpectationResolvedOk::InsSort(value)
    }
}

impl<'a> ExprTypeEngine<'a> {
    /// try to tell if a term is an instance of `Type u` for some universe u
    pub(super) fn resolve_ins_sort_expectation(
        &self,
        smallest_universe: TermUniverse,
        expectee: LocalTerm,
    ) -> Option<LocalTermExpectationResolvedOkM> {
        match expectee {
            LocalTerm::Resolved(resolved_expectee) => {
                let expectee_ty = self.db().term_ty(resolved_expectee);
                Some(match expectee_ty {
                    Ok(expectee_ty) => match expectee_ty.term() {
                        Term::Category(cat) => match cat.universe() >= smallest_universe {
                            true => LocalTermExpectationResolvedOkM {
                                result: Ok(LocalTermExpectationResolvedOk::InsSort(
                                    ExpectInsSortResolvedOk {
                                        destination: expectee,
                                    },
                                )),
                                actions: vec![],
                            },
                            false => LocalTermExpectationResolvedOkM {
                                result: Err(todo!()),
                                actions: vec![],
                            },
                        },
                        _ => LocalTermExpectationResolvedOkM {
                            result: Err(todo!()),
                            actions: vec![],
                        },
                    },
                    Err(error) => LocalTermExpectationResolvedOkM {
                        result: Err(error.into()),
                        actions: vec![],
                    },
                })
            }
            LocalTerm::Unresolved(_) => None,
        }
    }
}
