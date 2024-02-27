use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectSort {
    pub(crate) smallest_universe: Universe,
}

impl ExpectSort {
    pub const ANY: Self = Self {
        smallest_universe: Universe::new(0),
    };

    pub const TYPE: Self = Self {
        smallest_universe: Universe::new(1),
    };
}

impl ExpectFlyTerm for ExpectSort {
    type Outcome = Universe;

    #[inline(always)]
    fn retrieve_outcome(outcome: &ExpectationOutcome) -> &Self::Outcome {
        match outcome {
            ExpectationOutcome::EqsSort(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FlyTerms) -> FinalDestination {
        FinalDestination::Sort
    }

    #[inline(always)]
    fn destination(&self) -> FlyTermDestination {
        // todo: refine
        FlyTermDestination::AnyOriginal
    }

    fn resolve(
        &self,
        db: &::salsa::Db,
        terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        match state.expectee().base_resolved_inner(terms) {
            FlyTermBase::Eth(EthTerm::Category(cat)) => {
                match cat.universe() >= self.smallest_universe {
                    true => state.set_ok(ExpectationOutcome::EqsSort(cat.universe()), smallvec![]),
                    false => todo!(),
                    // state.set_err(todo!(), smallvec![]),
                }
            }
            _ => state.set_err(
                OriginalFlyTermExpectationError::ExpectedCategory {
                    expectee: state.expectee(),
                },
                smallvec![],
            ),
        }
    }
}
