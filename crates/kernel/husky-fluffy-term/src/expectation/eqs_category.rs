use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectEqsCategory {
    pub(crate) smallest_universe: UniverseTerm,
}

impl ExpectEqsCategory {
    pub const ANY_SORT: Self = Self {
        smallest_universe: UniverseTerm::new(0),
    };

    #[inline(always)]
    pub fn new_any_sort() -> Self {
        Self {
            smallest_universe: UniverseTerm::new(0),
        }
    }

    #[inline(always)]
    pub fn new_expect_eqs_ty_kind() -> Self {
        Self {
            smallest_universe: UniverseTerm::new(1),
        }
    }
}

impl ExpectFlyTerm for ExpectEqsCategory {
    type Outcome = UniverseTerm;

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
    fn destination(&self) -> Option<FlyTerm> {
        None
    }

    fn resolve(
        &self,
        db: &::salsa::Db,
        terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        match state.expectee().base_resolved_inner(terms) {
            FlyTermBase::Ethereal(EthTerm::Category(cat)) => {
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
