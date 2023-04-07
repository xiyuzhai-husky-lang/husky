use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TermDb)]
pub struct ExpectInsSort {
    smallest_universe: TermUniverse,
}

impl ExpectLocalTerm for ExpectInsSort {
    type Outcome = ExpectInsSortOutcome;

    fn retrieve_outcome(outcome: &LocalTermExpectationOutcome) -> &Self::Outcome {
        match outcome {
            LocalTermExpectationOutcome::InsSort(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(
        &self,
        db: &dyn TermDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        todo!()
    }

    fn destination(&self) -> Option<LocalTerm> {
        None
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
#[salsa::derive_debug_with_db(db = TermDb)]
pub struct ExpectInsSortOutcome {
    destination: LocalTerm,
}

impl ExpectInsSortOutcome {
    pub(crate) fn resolved(&self) -> Option<Term> {
        todo!()
    }
}

impl ExpectInsSort {
    /// try to tell if a term is an instance of `Type u` for some universe u
    pub(super) fn resolve(
        &self,
        db: &dyn TermDb,
        unresolved_terms: &mut UnresolvedTerms,
        expectee: LocalTerm,
    ) -> Option<LocalTermExpectationEffect> {
        match expectee {
            LocalTerm::Term(resolved_expectee) => {
                todo!()
                // let expectee_ty = term_ty(
                //     self.db(),
                //     todo!(),
                //     resolved_expectee,
                //     self.toolchain(),
                //     self.term_menu(),
                // );
                // Some(match expectee_ty {
                //     Ok(expectee_ty) => match expectee_ty {
                //         Term::Category(cat) => {
                //             match cat.universe() >= expectation.smallest_universe {
                //                 true => LocalTermExpectationEffect {
                //                     result: Ok(LocalTermExpectationOutcome::InsSort(
                //                         ExpectInsSortOutcome {
                //                             destination: expectee,
                //                         },
                //                     )),
                //                     actions: vec![],
                //                 },
                //                 false => LocalTermExpectationEffect {
                //                     result: Err(todo!()),
                //                     actions: vec![],
                //                 },
                //             }
                //         }
                //         _ => LocalTermExpectationEffect {
                //             result: Err(todo!()),
                //             actions: vec![],
                //         },
                //     },
                //     Err(error) => LocalTermExpectationEffect {
                //         result: Err(DerivedLocalTermExpectationError::TermTypeError {
                //             term: resolved_expectee,
                //             error,
                //         }
                //         .into()),
                //         actions: vec![],
                //     },
                // })
            }
            LocalTerm::Unresolved(_) => None,
            _ => todo!(),
        }
    }
}
