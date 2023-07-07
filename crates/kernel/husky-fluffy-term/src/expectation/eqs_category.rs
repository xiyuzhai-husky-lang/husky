use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectEqsCategory {
    pub(crate) smallest_universe: TermUniverse,
}

impl ExpectEqsCategory {
    pub const ANY_SORT: Self = Self {
        smallest_universe: TermUniverse::new(0),
    };

    #[inline(always)]
    pub fn new_any_sort() -> Self {
        Self {
            smallest_universe: TermUniverse::new(0),
        }
    }

    #[inline(always)]
    pub fn new_expect_eqs_ty_kind() -> Self {
        Self {
            smallest_universe: TermUniverse::new(1),
        }
    }
}

impl ExpectFluffyTerm for ExpectEqsCategory {
    type Outcome = TermUniverse;

    #[inline(always)]
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

    #[inline(always)]
    fn destination(&self) -> Option<FluffyTerm> {
        None
    }

    fn resolve(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> Option<ExpectationEffect> {
        match state.expectee() {
            FluffyTerm::Hollow(_) => todo!(),
            FluffyTerm::Solid(_) => todo!(),
            FluffyTerm::Category(cat) => match cat.universe() >= self.smallest_universe {
                true => state.set_ok(
                    FluffyTermExpectationOutcome::EqsSort(cat.universe()),
                    smallvec![],
                ),
                false => todo!(),
                // state.set_err(todo!(), smallvec![]),
            },
            _ => state.set_err(
                OriginalFluffyTermExpectationError::ExpectedCategory {
                    expectee: state.expectee(),
                },
                smallvec![],
            ),
        }
    }
}
