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

    fn retrieve_outcome(outcome: &FluffyTermExpectationOutcome) -> &Self::Outcome {
        match outcome {
            FluffyTermExpectationOutcome::EqsSort(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(
        &self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> FinalDestination {
        FinalDestination::Sort
    }

    fn destination(&self) -> Option<FluffyTerm> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExpectEqsSortOutcome {
    destination: FluffyTerm,
}

impl ExpectEqsCategory {
    pub(super) fn resolve(
        &self,
        db: &dyn FluffyTermDb,
        expectee: FluffyTerm,
        porous_terms: &mut FluffyTerms,
    ) -> Option<FluffyTermExpectationEffect> {
        todo!()
        // match expectee {
        //     FluffyTerm::Term(resolved_expectee) => match resolved_expectee {
        //         Term::Category(cat) => Some(match cat.universe() >= self.smallest_universe {
        //             true => FluffyTermExpectationEffect {
        //                 result: Ok(FluffyTermExpectationOutcome::EqsSort(cat.universe())),
        //                 actions: smallvec![],
        //             },
        //             false => FluffyTermExpectationEffect {
        //                 result: Err(todo!()),
        //                 actions: smallvec![],
        //             },
        //         }),
        //         _ => Some(FluffyTermExpectationEffect {
        //             result: Err(OriginalFluffyTermExpectationError::Todo.into()),
        //             actions: smallvec![],
        //         }),
        //     },
        //     FluffyTerm::Hollow(_) => todo!(),
        //     _ => todo!(),
        // }
    }
}
