use super::*;

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
    fn retrieve_outcome(outcome: &FluffyTermExpectationOutcome) -> &Self::Outcome {
        match outcome {
            FluffyTermExpectationOutcome::EqsExactly(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(
        &self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> FinalDestination {
        self.expected.final_destination_inner(db, terms)
    }

    #[inline(always)]
    fn destination(&self) -> Option<FluffyTerm> {
        Some(self.expected)
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

impl ExpectSubtype {
    pub(super) fn resolve(
        &self,
        db: &dyn FluffyTermDb,
        fluffy_terms: &mut FluffyTerms,
        expectee: FluffyTerm,
    ) -> Option<FluffyTermExpectationEffect> {
        if expectee == self.expected {
            return Some(FluffyTermExpectationEffect {
                result: Ok(ExpectSubtypeOutcome {}.into()),
                actions: smallvec![],
            });
        }
        match self.expected.data_inner(db, fluffy_terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                path: expected_path,
                arguments: argument_tys,
                ..
            } => match expectee.data_inner(db, fluffy_terms) {
                FluffyTermData::TypeOntology {
                    path: expectee_path,
                    arguments: argument_tys,
                    ..
                } => {
                    if expected_path == expectee_path {
                        todo!()
                    } else {
                        Some(FluffyTermExpectationEffect {
                            result: Err(OriginalFluffyTermExpectationError::TypePathMismatch {
                                expected_path,
                                expectee_path,
                            }
                            .into()),
                            actions: smallvec![],
                        })
                    }
                }
                FluffyTermData::Hole(_, _) => todo!(),
                _ => Some(FluffyTermExpectationEffect {
                    result: Err(todo!()),
                    actions: smallvec![],
                }),
            },
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
            } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => Some(FluffyTermExpectationEffect {
                result: Err(
                    OriginalFluffyTermExpectationError::ExpectedSubtype { expectee }.into(),
                ),
                actions: smallvec![],
            }),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => todo!(),
            FluffyTermData::PlaceTypeOntology { .. } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
        }
    }
}
