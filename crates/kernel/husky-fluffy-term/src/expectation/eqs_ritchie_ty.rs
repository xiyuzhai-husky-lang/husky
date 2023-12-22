use husky_coword::{IdentMap, IdentPairMap};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::debug_with_db]
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
    fn retrieve_outcome(outcome: &ExpectationOutcome) -> &Self::Outcome {
        match outcome {
            ExpectationOutcome::EqsRitchieCallType(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FluffyTerms) -> FinalDestination {
        self.final_destination
    }

    #[inline(always)]
    fn destination(&self) -> Option<FluffyTerm> {
        None
    }

    fn resolve(
        &self,
        db: &::salsa::Db,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FluffyTermEffect> {
        match state.expectee().data_inner(db, terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                ty_arguments: arguments,
                ..
            } => todo!(),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => {
                p!(curry_kind);
                todo!()
            }
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => state.set_ok(
                ExpectEqsRitchieTypeOutcome {
                    ritchie_kind,
                    parameter_contracted_tys: parameter_contracted_tys.to_smallvec(),
                    return_ty,
                },
                Default::default(),
            ),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::Rune { .. } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
// #[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ExpectEqsRitchieTypeOutcome {
    pub(crate) ritchie_kind: RitchieKind,
    pub(crate) parameter_contracted_tys: SmallVec<[FluffyRitchieParameter; 2]>,
    pub(crate) return_ty: FluffyTerm,
}

impl ExpectEqsRitchieTypeOutcome {
    pub fn parameter_contracted_tys(&self) -> &[FluffyRitchieParameter] {
        &self.parameter_contracted_tys
    }

    pub fn return_ty(&self) -> FluffyTerm {
        self.return_ty
    }

    pub fn ritchie_kind(&self) -> RitchieKind {
        self.ritchie_kind
    }
}
