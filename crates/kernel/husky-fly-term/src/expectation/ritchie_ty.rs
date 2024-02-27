use super::*;
use husky_term_prelude::ritchie::RitchieTypeKind;

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

impl ExpectFlyTerm for ExpectEqsRitchieType {
    type Outcome = ExpectEqsRitchieTypeOutcome;

    #[inline(always)]
    fn retrieve_outcome(outcome: &ExpectationOutcome) -> &Self::Outcome {
        match outcome {
            ExpectationOutcome::EqsRitchieCallType(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FlyTerms) -> FinalDestination {
        self.final_destination
    }

    #[inline(always)]
    fn destination(&self) -> FlyTermDestination {
        // todo: refine
        FlyTermDestination::AnyOriginal
    }

    fn resolve(
        &self,
        db: &::salsa::Db,
        terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        match state.expectee().base_ty_data_inner(db, terms) {
            FlyBaseTypeData::Ritchie {
                ritchie_kind: RitchieKind::Type(ritchie_ty_kind),
                parameter_contracted_tys,
                return_ty,
            } => state.set_ok(
                ExpectEqsRitchieTypeOutcome {
                    ritchie_ty_kind,
                    parameter_contracted_tys: parameter_contracted_tys.to_smallvec(),
                    return_ty,
                },
                Default::default(),
            ),
            _ => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
// #[salsa::derive_debug_with_db(db = FlyTermDb)]
pub struct ExpectEqsRitchieTypeOutcome {
    pub(crate) ritchie_ty_kind: RitchieTypeKind,
    pub(crate) parameter_contracted_tys: SmallVec<[FlyRitchieParameter; 2]>,
    pub(crate) return_ty: FlyTerm,
}

impl ExpectEqsRitchieTypeOutcome {
    pub fn parameter_contracted_tys(&self) -> &[FlyRitchieParameter] {
        &self.parameter_contracted_tys
    }

    pub fn return_ty(&self) -> FlyTerm {
        self.return_ty
    }

    pub fn ritchie_ty_kind(&self) -> RitchieTypeKind {
        self.ritchie_ty_kind
    }
}
