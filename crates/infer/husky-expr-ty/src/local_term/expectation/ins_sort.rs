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
    type Result = ExpectInsSortResult;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectInsSortResult {}
impl ExpectInsSortResult {
    pub(crate) fn resolved(&self) -> Option<ReducedTerm> {
        todo!()
    }
}

impl From<ExpectInsSort> for LocalTermExpectation {
    fn from(value: ExpectInsSort) -> Self {
        LocalTermExpectation::InsSort {
            smallest_universe: value.smallest_universe,
        }
    }
}

impl From<ExpectInsSortResult> for LocalTermExpectationResult {
    fn from(value: ExpectInsSortResult) -> Self {
        LocalTermExpectationResult::OkInsSort(value)
    }
}

impl<'a> ExprTypeEngine<'a> {
    /// try to tell if a term is an instance of `Type u` for some universe u
    pub(super) fn resolve_ins_sort_expectation(
        &self,
        smallest_universe: TermUniverse,
        expectee: LocalTerm,
    ) -> Option<LocalTermExpectationResultM> {
        match expectee {
            LocalTerm::Resolved(expectee) => {
                let expectee_ty = self.db().term_ty(expectee);
                Some(match expectee_ty {
                    Ok(expectee_ty) => match expectee_ty.term() {
                        Term::Category(cat) => match cat.universe() >= smallest_universe {
                            true => LocalTermExpectationResultM {
                                result: LocalTermExpectationResult::OkInsSort(
                                    ExpectInsSortResult {},
                                ),
                                actions: vec![],
                            },
                            false => LocalTermExpectationResultM {
                                result: LocalTermExpectationResult::Err(todo!()),
                                actions: vec![],
                            },
                        },
                        _ => LocalTermExpectationResultM {
                            result: LocalTermExpectationResult::Err(todo!()),
                            actions: vec![],
                        },
                    },
                    Err(_) => LocalTermExpectationResultM {
                        result: LocalTermExpectationResult::Err(todo!()),
                        actions: vec![],
                    },
                })
            }
            LocalTerm::Unresolved(_) => todo!(),
        }
    }
}
