use super::*;
use husky_declarative_ty::variance::HasVariances;

/// expect term to be equal to `Type` i.e. `Sort 1`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectSubtype {
    pub(crate) expected: FluffyTerm,
}

impl ExpectSubtype {
    pub fn new(destination: FluffyTerm) -> Self {
        Self {
            expected: destination,
        }
    }
}

impl ExpectFluffyTerm for ExpectSubtype {
    type Outcome = ExpectSubtypeOutcome;

    #[inline(always)]
    fn retrieve_outcome(outcome: &ExpectationOutcome) -> &Self::Outcome {
        match outcome {
            ExpectationOutcome::Subtype(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FluffyTerms) -> FinalDestination {
        self.expected.final_destination_inner(db, terms)
    }

    #[inline(always)]
    fn destination(&self) -> Option<FluffyTerm> {
        Some(self.expected)
    }

    // todo: use ty_data instead
    fn resolve(
        &self,
        db: &::salsa::Db,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FluffyTermEffect> {
        if state.expectee() == self.expected {
            return state.set_ok(ExpectSubtypeOutcome {}, smallvec![]);
        }
        match self.expected.data_inner(db, terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path: expected_ty_path,
                ty_arguments: expected_ty_arguments,
                ..
            } => match state.expectee().data_inner(db, terms) {
                FluffyTermData::TypeOntology {
                    ty_path: expectee_ty_path,
                    ty_arguments: expectee_ty_arguments,
                    ..
                } => {
                    if expected_ty_path == expectee_ty_path {
                        let ty_path = expected_ty_path;
                        assert_eq!(expected_ty_arguments.len(), expected_ty_arguments.len());
                        let variances = match ty_path.variances(db) {
                            Ok(variances) => variances,
                            Err(_) => todo!(),
                        };
                        state.set_ok(
                            ExpectSubtypeOutcome {},
                            std::iter::zip(
                                variances,
                                std::iter::zip(expectee_ty_arguments, expected_ty_arguments),
                            )
                            .map(
                                |(&variance, (&expectee_ty_argument, &expected_ty_argument))| {
                                    match variance {
                                        // ad hoc
                                        Variance::Independent => {
                                            FluffyTermResolveAction::AddExpectation {
                                                src: state.child_src(),
                                                expectee: expectee_ty_argument,
                                                expectation: ExpectSubtype {
                                                    expected: expected_ty_argument,
                                                }
                                                .into(),
                                            }
                                        }
                                        Variance::Covariant => {
                                            FluffyTermResolveAction::AddExpectation {
                                                src: state.child_src(),
                                                expectee: expectee_ty_argument,
                                                expectation: ExpectSubtype {
                                                    expected: expected_ty_argument,
                                                }
                                                .into(),
                                            }
                                        }
                                        Variance::Contravariant => {
                                            FluffyTermResolveAction::AddExpectation {
                                                src: state.child_src(),
                                                expectee: expected_ty_argument,
                                                expectation: ExpectSubtype {
                                                    expected: expectee_ty_argument,
                                                }
                                                .into(),
                                            }
                                        }
                                        Variance::Invariant => todo!(),
                                    }
                                },
                            )
                            .collect(),
                        )
                    } else {
                        state.set_err(
                            OriginalFluffyTermExpectationError::TypePathMismatchForSubtyping {
                                expected: self.expected,
                                expectee: state.expectee(),
                                expected_path: expected_ty_path,
                                expectee_path: expectee_ty_path,
                            },
                            smallvec![],
                        )
                    }
                }
                FluffyTermData::Hole(_, hole) => {
                    if Into::<FluffyTerm>::into(hole) != state.expectee() {
                        state.set_holed(hole, |state| HoleConstraint::CoercibleInto {
                            target: state.expectee(),
                        })
                    } else {
                        state.set_ok(ExpectSubtypeOutcome {}, smallvec![])
                    }
                }
                _ => todo!(), // Some(FluffyTermExpectationEffect {
                              //     result: Err(todo!()),
                              //     actions: smallvec![],
                              // }),
            },
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Hole(_, hole) => {
                state.set_ok(
                    ExpectSubtypeOutcome {},
                    smallvec![FluffyTermResolveAction::FillHole {
                        // todo: check hole kind
                        hole,
                        // todo: check subtype
                        term: state.expectee()
                    }],
                )
            }
            FluffyTermData::Category(_) => state.set_err(
                OriginalFluffyTermExpectationError::ExpectedSubtype {
                    expectee: state.expectee(),
                },
                smallvec![],
            ),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => todo!(),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::Rune { .. } => todo!(),
            FluffyTermData::TypeVariant { path } => match state.expectee().data_inner(db, terms) {
                FluffyTermData::Literal(_) => todo!(),
                FluffyTermData::TypeOntology {
                    ty_path,
                    refined_ty_path,
                    ty_arguments: arguments,
                    ty_ethereal_term,
                } => todo!(),
                FluffyTermData::Curry {
                    curry_kind,
                    variance,
                    parameter_rune: parameter_rune,
                    parameter_ty,
                    return_ty,
                    ty_ethereal_term,
                } => todo!(),
                FluffyTermData::Hole(_, hole) => state.set_ok(
                    ExpectSubtypeOutcome {},
                    smallvec![FluffyTermResolveAction::FillHole {
                        hole,
                        term: self.expected,
                    }],
                ),
                FluffyTermData::Category(_) => todo!(),
                FluffyTermData::Ritchie {
                    ritchie_kind,
                    parameter_contracted_tys,
                    return_ty,
                } => todo!(),
                FluffyTermData::Symbol { term, ty } => todo!(),
                FluffyTermData::Rune { .. } => todo!(),
                FluffyTermData::TypeVariant { path } => todo!(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectSubtypeOutcome {
    // todo: change this to option lifetime subtype constraint
}

impl ExpectSubtypeOutcome {
    pub(crate) fn resolved(&self) -> Option<EtherealTerm> {
        todo!()
    }
}
