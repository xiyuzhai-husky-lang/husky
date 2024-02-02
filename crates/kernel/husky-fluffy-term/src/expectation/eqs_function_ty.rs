use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::debug_with_db]
pub struct ExpectEqsFunctionType {
    final_destination: FinalDestination,
}

impl ExpectEqsFunctionType {
    pub fn new(final_destination: FinalDestination) -> Self {
        Self { final_destination }
    }
}

impl ExpectFlyTerm for ExpectEqsFunctionType {
    type Outcome = ExpectEqsFunctionTypeOutcome;

    #[inline(always)]
    fn retrieve_outcome(outcome: &ExpectationOutcome) -> &Self::Outcome {
        match outcome {
            ExpectationOutcome::EqsFunctionCallType(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FlyTerms) -> FinalDestination {
        self.final_destination
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
        match state.expectee().data_inner(db, terms) {
            FlyTermData::Literal(_) => todo!(),
            FlyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                ty_arguments: arguments,
                ..
            } => state.set_err(
                OriginalFlyTermExpectationError::ExpectedFunctionType,
                smallvec![],
            ),
            FlyTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => state.set_ok(
                ExpectEqsFunctionTypeOutcome {
                    return_ty,
                    variant: ExpectEqsFunctionTypeOutcomeData::Curry {
                        variance,
                        parameter_rune,
                        parameter_ty,
                        return_ty,
                    },
                },
                smallvec![],
            ),
            FlyTermData::Hole(_, _) => todo!(),
            FlyTermData::Category(_) => state.set_err(
                OriginalFlyTermExpectationError::ExpectedFunctionType,
                smallvec![],
            ),
            FlyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => state.set_ok(
                ExpectEqsFunctionTypeOutcome {
                    return_ty,
                    variant: ExpectEqsFunctionTypeOutcomeData::Ritchie {
                        ritchie_kind,
                        parameter_contracted_tys: parameter_contracted_tys.to_vec(),
                    },
                },
                smallvec![],
            ),
            FlyTermData::Symbol { .. } => todo!(),
            FlyTermData::Rune { .. } => todo!(),
            FlyTermData::TypeVariant { path } => todo!(),
        }
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExpectEqsFunctionTypeOutcome {
    pub(crate) return_ty: FlyTerm,
    pub(crate) variant: ExpectEqsFunctionTypeOutcomeData,
}

impl ExpectEqsFunctionTypeOutcome {
    pub fn variant(&self) -> &ExpectEqsFunctionTypeOutcomeData {
        &self.variant
    }

    pub fn return_ty(&self) -> FlyTerm {
        self.return_ty
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExpectEqsFunctionTypeOutcomeData {
    Ritchie {
        ritchie_kind: RitchieKind,
        parameter_contracted_tys: Vec<FlyRitchieParameter>,
    },
    Curry {
        variance: Variance,
        parameter_rune: Option<RuneFlyTerm>,
        parameter_ty: FlyTerm,
        return_ty: FlyTerm,
    },
}
