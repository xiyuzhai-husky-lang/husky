use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ExpectEqsFunctionType {
    final_destination: FinalDestination,
}

impl ExpectEqsFunctionType {
    pub fn new(final_destination: FinalDestination) -> Self {
        Self { final_destination }
    }
}

impl ExpectFluffyTerm for ExpectEqsFunctionType {
    type Outcome = ExpectEqsFunctionTypeOutcome;

    #[inline(always)]
    fn retrieve_outcome(outcome: &FluffyTermExpectationOutcome) -> &Self::Outcome {
        match outcome {
            FluffyTermExpectationOutcome::EqsFunctionCallType(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(
        &self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> FinalDestination {
        self.final_destination
    }

    #[inline(always)]
    fn destination(&self) -> Option<FluffyTerm> {
        None
    }

    fn resolve(
        &self,
        db: &dyn FluffyTermDb,
        state: &mut ExpectationMeta,
        terms: &mut FluffyTerms,
    ) -> Option<ExpectationEffect> {
        // todo: move these to aux
        match state.expectee().data_inner(db, terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                arguments,
                ..
            } => state.set_err(
                OriginalFluffyTermExpectationError::ExpectedFunctionType,
                smallvec![],
            ),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => self.resolve_curry(
                db,
                state,
                terms,
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
            ),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => state.set_err(
                OriginalFluffyTermExpectationError::ExpectedFunctionType,
                smallvec![],
            ),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => state.set_ok(
                ExpectEqsFunctionTypeOutcome {
                    implicit_parameter_substitutions: smallvec![],
                    return_ty,
                    variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                        ritchie_kind,
                        parameter_contracted_tys: parameter_contracted_tys.to_vec(),
                    },
                },
                smallvec![],
            ),
            FluffyTermData::PlaceTypeOntology { .. } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
            FluffyTermData::Symbol { ty } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ExpectEqsFunctionTypeOutcome {
    pub(crate) implicit_parameter_substitutions: SmallVec<[ImplicitParameterSubstitution; 2]>,
    pub(crate) return_ty: FluffyTerm,
    pub(crate) variant: ExpectEqsFunctionTypeOutcomeVariant,
}

impl ExpectEqsFunctionTypeOutcome {
    pub fn variant(&self) -> &ExpectEqsFunctionTypeOutcomeVariant {
        &self.variant
    }

    pub fn return_ty(&self) -> FluffyTerm {
        self.return_ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub enum ExpectEqsFunctionTypeOutcomeVariant {
    Ritchie {
        ritchie_kind: RitchieKind,
        parameter_contracted_tys: Vec<FluffyTermRitchieParameterContractedType>,
    },
    Curry {
        variance: Variance,
        parameter_symbol: Option<FluffyTerm>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
    },
}

impl ExpectEqsFunctionType {
    fn resolve_curry(
        &self,
        db: &dyn FluffyTermDb,
        state: &mut ExpectationMeta,
        terms: &mut FluffyTerms,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_symbol: Option<FluffyTerm>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
    ) -> Option<ExpectationEffect> {
        match curry_kind {
            CurryKind::Explicit => state.set_ok(
                ExpectEqsFunctionTypeOutcome {
                    implicit_parameter_substitutions: smallvec![],
                    return_ty,
                    variant: ExpectEqsFunctionTypeOutcomeVariant::Curry {
                        variance,
                        parameter_symbol,
                        parameter_ty,
                        return_ty,
                    },
                },
                smallvec![],
            ),
            CurryKind::Implicit => match parameter_symbol {
                Some(parameter_symbol) => {
                    let implicit_symbol = terms.new_hole_from_parameter_symbol(
                        db,
                        HoleSource::Expectation(state.idx()),
                        parameter_symbol,
                    );
                    let mut implicit_parameter_substitutions =
                        smallvec![ImplicitParameterSubstitution::new(
                            parameter_symbol,
                            implicit_symbol,
                        )];
                    let expectee = return_ty;
                    let expectee = terms.substitute_into_term(
                        db,
                        HoleSource::Expectation(state.idx()),
                        expectee,
                        &implicit_parameter_substitutions,
                    );
                    self.resolve_aux(db, state, terms, expectee, implicit_parameter_substitutions)
                }
                None => todo!(), // self.resolve(db, terms, idx, return_ty),
            },
        }
    }

    fn resolve_aux(
        &self,
        db: &dyn FluffyTermDb,
        state: &mut ExpectationMeta,
        terms: &mut FluffyTerms,
        expectee: FluffyTerm,
        mut substitution_rules: SmallVec<[ImplicitParameterSubstitution; 2]>,
    ) -> Option<ExpectationEffect> {
        match expectee.data_inner(db, terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                arguments,
                ..
            } => todo!(),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable: parameter_symbol,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => state.set_ok(
                ExpectEqsFunctionTypeOutcome {
                    // todo: is this really correct?
                    implicit_parameter_substitutions: substitution_rules,
                    return_ty,
                    variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                        ritchie_kind,
                        parameter_contracted_tys: parameter_contracted_tys.to_vec(),
                    },
                },
                smallvec![],
            ),
            FluffyTermData::PlaceTypeOntology { .. } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
            FluffyTermData::Symbol { ty } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
        }
    }
}
