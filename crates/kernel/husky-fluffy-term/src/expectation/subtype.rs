use super::*;
use husky_declarative_ty::variance::HasVariances;

/// expect term to be equal to `Type` i.e. `Sort 1`
#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectSubtype {
    pub(crate) expected: FlyTerm,
}

impl ExpectSubtype {
    pub fn new(destination: FlyTerm) -> Self {
        Self {
            expected: destination,
        }
    }
}

impl ExpectFlyTerm for ExpectSubtype {
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
    fn destination(&self) -> Option<FlyTerm> {
        Some(self.expected)
    }

    // todo: use ty_data instead
    fn resolve(
        &self,
        db: &::salsa::Db,
        terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        let expectee_data = state.expectee().data_inner(db, terms);
        if expectee_data == self.expected.data_inner(db, terms) {
            return state.set_ok(ExpectSubtypeOutcome {}, smallvec![]);
        }
        // todo: handle the case that expectee is a hole first, like
        // ```
        // match state.expectee().data_inner(db, terms) {
        //     FlyTermData::Hole(_, hole) => {
        //         state.set_holed(hole, |state| HoleConstraint::CoercibleInto {
        //             target: self.expected,
        //         })
        //     }
        //     _ => (),
        // }
        // ```
        match self.expected.data_inner(db, terms) {
            FlyTermData::Literal(_) => todo!(),
            FlyTermData::TypeOntology {
                ty_path: expected_ty_path,
                ty_arguments: expected_ty_arguments,
                ..
            } => match state.expectee().data_inner(db, terms) {
                FlyTermData::TypeOntology {
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
                                            FlyTermResolveAction::AddExpectation {
                                                src: state.child_src(),
                                                expectee: expectee_ty_argument,
                                                expectation: ExpectSubtype {
                                                    expected: expected_ty_argument,
                                                }
                                                .into(),
                                            }
                                        }
                                        Variance::Covariant => {
                                            FlyTermResolveAction::AddExpectation {
                                                src: state.child_src(),
                                                expectee: expectee_ty_argument,
                                                expectation: ExpectSubtype {
                                                    expected: expected_ty_argument,
                                                }
                                                .into(),
                                            }
                                        }
                                        Variance::Contravariant => {
                                            FlyTermResolveAction::AddExpectation {
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
                            OriginalFlyTermExpectationError::TypePathMismatchForSubtyping {
                                expected: self.expected,
                                expectee: state.expectee(),
                                expected_path: expected_ty_path,
                                expectee_path: expectee_ty_path,
                            },
                            smallvec![],
                        )
                    }
                }
                FlyTermData::Hole(_, hole) => {
                    state.set_holed(hole, |state| HoleConstraint::CoercibleInto {
                        target: self.expected,
                    })
                }
                expectee_data => {
                    p!(self.expected.show(db, terms), expectee_data.debug(db));
                    todo!()
                } // Some(FlyTermExpectationEffect {
                  //     result: Err(todo!()),
                  //     actions: smallvec![],
                  // }),
            },
            FlyTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FlyTermData::Hole(_, hole) => {
                state.set_ok(
                    ExpectSubtypeOutcome {},
                    smallvec![FlyTermResolveAction::FillHole {
                        // todo: check hole kind
                        hole,
                        // todo: check subtype
                        term: state.expectee()
                    }],
                )
            }
            FlyTermData::Category(_) => state.set_err(
                OriginalFlyTermExpectationError::ExpectedSubtype {
                    expectee: state.expectee(),
                },
                smallvec![],
            ),
            FlyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => todo!(),
            FlyTermData::Symbol {
                ty: expected_ty, ..
            } => match state.expectee().base_ty_data_inner(db, terms) {
                FlyBaseTypeData::TypeOntology {
                    ty_path,
                    refined_ty_path,
                    ty_arguments,
                    ty_ethereal_term,
                } => todo!(),
                FlyBaseTypeData::Curry {
                    curry_kind,
                    variance,
                    parameter_rune,
                    parameter_ty,
                    return_ty,
                    ty_ethereal_term,
                } => todo!(),
                FlyBaseTypeData::Hole(hole_kind, hole) => match hole_kind {
                    HoleKind::UnspecifiedIntegerType => todo!(),
                    HoleKind::UnspecifiedFloatType => todo!(),
                    HoleKind::ImplicitType => match expected_ty.base_resolved_inner(terms) {
                        FlyTermBase::Ethereal(EthTerm::Category(_)) => state.set_ok(
                            ExpectSubtypeOutcome {},
                            smallvec![FlyTermResolveAction::FillHole {
                                hole,
                                term: self.expected
                            }],
                        ),
                        _ => todo!(),
                    },
                    HoleKind::Any => state.set_ok(
                        ExpectSubtypeOutcome {},
                        smallvec![FlyTermResolveAction::FillHole {
                            hole,
                            term: self.expected
                        }],
                    ),
                },
                FlyBaseTypeData::Category(_) => todo!(),
                FlyBaseTypeData::Ritchie {
                    ritchie_kind,
                    parameter_contracted_tys,
                    return_ty,
                } => todo!(),
                FlyBaseTypeData::Symbol { symbol } => todo!(),
                FlyBaseTypeData::Rune { rune } => todo!(),
            },
            FlyTermData::Rune { .. } => todo!(),
            FlyTermData::TypeVariant { path } => match state.expectee().data_inner(db, terms) {
                FlyTermData::Literal(_) => todo!(),
                FlyTermData::TypeOntology {
                    ty_path,
                    refined_ty_path,
                    ty_arguments: arguments,
                    ty_ethereal_term,
                } => todo!(),
                FlyTermData::Curry {
                    toolchain,
                    curry_kind,
                    variance,
                    parameter_rune,
                    parameter_ty,
                    return_ty,
                    ty_ethereal_term,
                } => todo!(),
                FlyTermData::Hole(_, hole) => state.set_ok(
                    ExpectSubtypeOutcome {},
                    smallvec![FlyTermResolveAction::FillHole {
                        hole,
                        term: self.expected,
                    }],
                ),
                FlyTermData::Category(_) => todo!(),
                FlyTermData::Ritchie {
                    ritchie_kind,
                    parameter_contracted_tys,
                    return_ty,
                } => todo!(),
                FlyTermData::Symbol { term, ty } => todo!(),
                FlyTermData::Rune { .. } => todo!(),
                FlyTermData::TypeVariant { path } => todo!(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectSubtypeOutcome {
    // todo: change this to option lifetime subtype constraint
}

impl ExpectSubtypeOutcome {
    pub(crate) fn resolved(&self) -> Option<EthTerm> {
        todo!()
    }
}
