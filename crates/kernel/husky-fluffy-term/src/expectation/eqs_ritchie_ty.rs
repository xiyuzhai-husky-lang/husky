use husky_word::{IdentMap, IdentPairMap};

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
}

#[derive(Debug, PartialEq, Eq, Clone)]
// #[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ExpectEqsRitchieTypeOutcome {
    pub(crate) ritchie_kind: RitchieKind,
    pub(crate) implicit_parameter_substitutions: SmallVec<[ImplicitParameterSubstitution; 2]>,
    pub(crate) parameter_contracted_tys: SmallVec<[FluffyTermRitchieParameterContractedType; 2]>,
    pub(crate) return_ty: FluffyTerm,
}

impl ExpectEqsRitchieTypeOutcome {
    pub fn parameter_contracted_tys(&self) -> &[FluffyTermRitchieParameterContractedType] {
        &self.parameter_contracted_tys
    }

    pub fn return_ty(&self) -> FluffyTerm {
        self.return_ty
    }
}

impl ExpectEqsRitchieType {
    pub(super) fn resolve(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        idx: FluffyTermExpectationIdx,
        expectee: FluffyTerm,
    ) -> Option<FluffyTermExpectationEffect> {
        // todo: move these to aux
        match expectee.data_inner(db, terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                path,
                refined_path,
                arguments,
                ..
            } => Some(FluffyTermExpectationEffect {
                result: Err(OriginalFluffyTermExpectationError::Todo.into()),
                actions: smallvec![],
            }),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable: parameter_symbol,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => self.resolve_curry(
                db,
                terms,
                idx,
                curry_kind,
                variance,
                parameter_symbol,
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
            } => Some(FluffyTermExpectationEffect {
                result: Ok(ExpectEqsRitchieTypeOutcome {
                    ritchie_kind,
                    implicit_parameter_substitutions: smallvec![],
                    parameter_contracted_tys: parameter_contracted_tys.to_smallvec(),
                    return_ty,
                }
                .into()),
                actions: smallvec![],
            }),
            FluffyTermData::PlaceTypeOntology { .. } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
        }
    }

    fn resolve_curry(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        idx: FluffyTermExpectationIdx,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_symbol: Option<FluffyTerm>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
    ) -> Option<FluffyTermExpectationEffect> {
        match curry_kind {
            CurryKind::Explicit => todo!(),
            CurryKind::Implicit => match parameter_symbol {
                Some(parameter_symbol) => {
                    let implicit_symbol = terms.new_hole_from_parameter_symbol(
                        db,
                        HoleSource::Expectation(idx),
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
                        HoleSource::Expectation(idx),
                        expectee,
                        &implicit_parameter_substitutions,
                    );
                    self.resolve_aux(db, terms, idx, expectee, implicit_parameter_substitutions)
                }
                None => self.resolve(db, terms, idx, return_ty),
            },
        }
    }

    fn resolve_aux(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        idx: FluffyTermExpectationIdx,
        expectee: FluffyTerm,
        mut substitution_rules: SmallVec<[ImplicitParameterSubstitution; 2]>,
    ) -> Option<FluffyTermExpectationEffect> {
        match expectee.data_inner(db, terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                path,
                refined_path,
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
            } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie { .. } => todo!(),
            FluffyTermData::PlaceTypeOntology { .. } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
        }
    }
}
