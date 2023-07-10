use husky_coword::{IdentMap, IdentPairMap};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ExpectEqsRitchieType {
    final_destination: FinalDestination,
}

impl ExpectEqsRitchieType {
    pub fn new(final_destination: FinalDestination) -> Self {
        Self { final_destination }
    }
}

impl ExpectFluffyTerm for ExpectEqsRitchieType {
    type Outcome = ExpectEqsRitchieTypeOutcome;

    #[inline(always)]
    fn retrieve_outcome(outcome: &FluffyTermExpectationOutcome) -> &Self::Outcome {
        match outcome {
            FluffyTermExpectationOutcome::EqsRitchieCallType(outcome) => outcome,
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
        terms: &mut FluffyTerms,
        meta: &mut ExpectationState,
    ) -> Option<ExpectationEffect> {
        // todo: move these to aux
        match meta.expectee().data_inner(db, terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                arguments,
                ..
            } => meta.set_err(OriginalFluffyTermExpectationError::Todo, smallvec![]),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => self.resolve_curry(
                db,
                terms,
                meta,
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
            ),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => meta.set_ok(
                ExpectEqsRitchieTypeOutcome {
                    ritchie_kind,
                    implicit_parameter_substitutions: smallvec![],
                    parameter_contracted_tys: parameter_contracted_tys.to_smallvec(),
                    return_ty,
                },
                smallvec![],
            ),
            FluffyTermData::TypeOntologyAtPlace { .. } => todo!(),
            FluffyTermData::HoleAtPlace {
                place,
                hole_kind,
                hole,
            } => todo!(),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::SymbolAtPlace { .. } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
// #[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ExpectEqsRitchieTypeOutcome {
    pub(crate) ritchie_kind: RitchieKind,
    pub(crate) implicit_parameter_substitutions: SmallVec<[ImplicitParameterSubstitution; 2]>,
    pub(crate) parameter_contracted_tys: SmallVec<[FluffyTermRitchieParameter; 2]>,
    pub(crate) return_ty: FluffyTerm,
}

impl ExpectEqsRitchieTypeOutcome {
    pub fn parameter_contracted_tys(&self) -> &[FluffyTermRitchieParameter] {
        &self.parameter_contracted_tys
    }

    pub fn return_ty(&self) -> FluffyTerm {
        self.return_ty
    }
}

impl ExpectEqsRitchieType {
    fn resolve_curry(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        meta: &mut ExpectationState,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_variable: Option<FluffyTerm>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
    ) -> Option<ExpectationEffect> {
        self.resolve_curry_aux(
            db,
            terms,
            meta,
            curry_kind,
            variance,
            parameter_variable,
            parameter_ty,
            return_ty,
            smallvec![],
        )
    }

    fn resolve_aux(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
        expectee: FluffyTerm,
        mut implicit_parameter_substitutions: SmallVec<[ImplicitParameterSubstitution; 2]>,
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
                parameter_variable,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => self.resolve_curry_aux(
                db,
                terms,
                state,
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
                implicit_parameter_substitutions,
            ),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => state.set_ok(
                ExpectEqsRitchieTypeOutcome {
                    ritchie_kind,
                    implicit_parameter_substitutions,
                    parameter_contracted_tys: parameter_contracted_tys.to_smallvec(),
                    return_ty,
                },
                Default::default(),
            ),
            FluffyTermData::TypeOntologyAtPlace { .. } => todo!(),
            FluffyTermData::HoleAtPlace {
                place,
                hole_kind,
                hole,
            } => todo!(),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::SymbolAtPlace { .. } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
        }
    }

    fn resolve_curry_aux(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        meta: &mut ExpectationState,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_variable: Option<FluffyTerm>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
        mut implicit_parameter_substitutions: SmallVec<[ImplicitParameterSubstitution; 2]>,
    ) -> Option<ExpectationEffect> {
        match curry_kind {
            CurryKind::Explicit => todo!(),
            // comes from implicit parameters, or generics in other languages
            CurryKind::Implicit => match parameter_variable {
                Some(parameter_variable) => {
                    let implicit_symbol = terms.new_hole_from_parameter_symbol(
                        db,
                        HoleSource::Expectation(meta.idx()),
                        parameter_variable,
                    );
                    implicit_parameter_substitutions.push(ImplicitParameterSubstitution::new(
                        parameter_variable,
                        implicit_symbol,
                    ));
                    let expectee = return_ty.rewrite_inner(
                        db,
                        terms,
                        HoleSource::Expectation(meta.idx()),
                        &implicit_parameter_substitutions,
                    );
                    self.resolve_aux(db, terms, meta, expectee, implicit_parameter_substitutions)
                }
                None => todo!(),
            },
        }
    }
}
