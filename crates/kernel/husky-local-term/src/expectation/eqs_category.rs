use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectEqsCategory {
    pub(crate) smallest_universe: TermUniverse,
}

impl ExpectEqsCategory {
    pub const ANY_SORT: Self = Self {
        smallest_universe: TermUniverse::new(0),
    };

    pub fn new_any_sort() -> Self {
        Self {
            smallest_universe: TermUniverse::new(0),
        }
    }

    pub fn new_expect_eqs_ty_kind() -> Self {
        Self {
            smallest_universe: TermUniverse::new(1),
        }
    }
}

impl ExpectLocalTerm for ExpectEqsCategory {
    type Outcome = TermUniverse;

    fn retrieve_outcome(outcome: &LocalTermExpectationOutcome) -> &Self::Outcome {
        match outcome {
            LocalTermExpectationOutcome::EqsSort(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination(
        &self,
        db: &dyn TermDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        FinalDestination::Sort
    }

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExpectEqsSortOutcome {
    destination: LocalTerm,
}

impl ExpectEqsCategory {
    pub(super) fn resolve(
        &self,
        db: &dyn TermDb,
        expectee: LocalTerm,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        match expectee {
            LocalTerm::Resolved(resolved_expectee) => match resolved_expectee {
                Term::Category(cat) => Some(match cat.universe() >= self.smallest_universe {
                    true => LocalTermExpectationEffect {
                        result: Ok(LocalTermExpectationOutcome::EqsSort(cat.universe())),
                        actions: smallvec![],
                    },
                    false => LocalTermExpectationEffect {
                        result: Err(todo!()),
                        actions: smallvec![],
                    },
                }),
                _ => Some(LocalTermExpectationEffect {
                    result: Err(OriginalLocalTermExpectationError::Todo.into()),
                    actions: smallvec![],
                }),
            },
            LocalTerm::Unresolved(_) => None,
        }
    }
}
