use super::*;
use husky_entity_path::path::major_item::ty::{PreludeNumTypePath, PreludeTypePath};

/// expect primitive number types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectIntType;

impl ExpectFlyTerm for ExpectIntType {
    type Outcome = ExpectIntTypeOutcome;

    #[inline(always)]
    fn retrieve_outcome(outcome: &ExpectationOutcome) -> &Self::Outcome {
        match outcome {
            ExpectationOutcome::IntType(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FlyTerms) -> FinalDestination {
        todo!()
    }

    #[inline(always)]
    fn destination(&self) -> FlyTermDestination {
        // todo: refine
        FlyTermDestination::AnyOriginal
    }

    fn resolve(
        &self,
        db: &::salsa::Db,
        fly_terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        let expectee = state.expectee();
        match expectee.base_ty_data_inner(db, fly_terms) {
            FlyBaseTypeData::TypeOntology {
                refined_ty_path: Left(PreludeTypePath::Num(PreludeNumTypePath::Int(_))),
                ..
            } => state.set_ok(
                ExpectIntTypeOutcome {
                    placeless_int_ty: state.expectee(),
                },
                smallvec![],
            ),
            FlyBaseTypeData::Hole(hole_kind, hole) => match hole_kind {
                HoleKind::UnspecifiedIntegerType | HoleKind::UnspecifiedFloatType => state.set_ok(
                    ExpectIntTypeOutcome {
                        placeless_int_ty: hole.into(),
                    },
                    smallvec![],
                ),
                HoleKind::ImplicitType | HoleKind::AnyOriginal | HoleKind::AnyDerived => AltNone,
            },
            _ => state.set_err(
                OriginalFlyTermExpectationError::ExpectedIntType { expectee },
                smallvec![],
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExpectIntTypeOutcome {
    /// guaranteed to be placeless
    placeless_int_ty: FlyTerm,
}

impl ExpectIntTypeOutcome {
    /// guaranteed to be placeless
    pub fn placeless_int_ty(&self) -> FlyTerm {
        self.placeless_int_ty
    }
}
