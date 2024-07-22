use std::iter::zip;

use super::*;
use husky_dec_ty::variance::HasVariances;

/// if terms aren't equal to `Type` i.e. `Sort 1`, this just menas terms are equal
#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectSubtypeOrEqual {
    pub(crate) expected: FlyTerm,
}

impl ExpectSubtypeOrEqual {
    pub fn new(destination: FlyTerm) -> Self {
        Self {
            expected: destination,
        }
    }
}

impl ExpectFlyTerm for ExpectSubtypeOrEqual {
    type Outcome = ExpectSubtypeOutcome;

    #[inline(always)]
    fn retrieve_outcome(outcome: &ExpectationOutcome) -> &Self::Outcome {
        match outcome {
            ExpectationOutcome::Subtype(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FlyTerms) -> FinalDestination {
        self.expected.final_destination_inner(db, terms)
    }

    #[inline(always)]
    fn destination(&self) -> FlyTermDestination {
        FlyTermDestination::Specific(self.expected)
    }

    // expectee should be a subtype of expecteed
    // todo: use ty_data instead
    fn resolve(
        &self,
        db: &::salsa::Db,
        terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        let expectee = state.expectee();
        let expected = self.expected;
        let t0 = expectee.base_term_data2(db, terms);
        let t1 = expected.base_term_data2(db, terms);
        match (t0, t1) {
            (t0, t1) if t0 == t1 => state.set_ok(ExpectSubtypeOutcome {}, smallvec![]),
            (FlyTermData::Hole(_, hole), _) => state.set_holed(hole, |_| HoleConstraint::Subtype {
                target: self.expected,
            }),
            (_, FlyTermData::Hole(_, hole)) => {
                state.set_holed(hole, |_| HoleConstraint::Supertype { target: expectee })
            }
            (
                FlyTermData::TypeOntology {
                    ty_path: ty_path0,
                    ty_arguments: args0,
                    ..
                },
                FlyTermData::TypeOntology {
                    ty_path: ty_path1,
                    ty_arguments: args1,
                    ..
                },
            ) => {
                if ty_path0 != ty_path1 {
                    return state.set_err(
                        OriginalFlyTermExpectationError::TypePathMismatchForSubtyping {
                            expected: self.expected,
                            expectee: state.expectee(),
                            expected_path: ty_path1,
                            expectee_path: ty_path0,
                        },
                        smallvec![],
                    );
                }
                let ty_path = ty_path0;
                assert_eq!(args0.len(), args1.len());
                let variances = match ty_path.variances(db) {
                    Ok(variances) => variances,
                    Err(_) => {
                        return state.set_err(DerivedFlyTermExpectationError::Variance, smallvec![])
                    }
                };
                let subsequent_actions = zip(variances, zip(args0, args1))
                    .map(|(&variance, (&arg0, &arg1))| {
                        let (expectee, expected) = match variance {
                            Variance::Independent => (arg0, arg1),
                            Variance::Covariant => (arg0, arg1),
                            Variance::Contravariant => (arg1, arg0),
                            Variance::Invariant => todo!(),
                        };
                        FlyTermResolveAction::AddExpectation {
                            src: state.child_src(),
                            expectee,
                            expectation: ExpectSubtypeOrEqual { expected }.into(),
                        }
                    })
                    .collect();
                state.set_ok(ExpectSubtypeOutcome {}, subsequent_actions)
            }
            (FlyTermData::Curry { .. }, FlyTermData::Curry { .. }) => todo!(),
            (FlyTermData::Ritchie { .. }, FlyTermData::Ritchie { .. }) => todo!(),
            _ => state.set_err(
                OriginalFlyTermExpectationError::ExpectedSubtype {
                    expectee: state.expectee(),
                },
                smallvec![],
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectSubtypeOutcome {
    // todo: change this to option lifetime subtype constraint
}
