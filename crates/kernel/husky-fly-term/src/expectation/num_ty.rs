use super::*;

/// expect primitive number types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectNumType;

impl ExpectFlyTerm for ExpectNumType {
    type Outcome = ExpectNumTypeOutcome;

    #[inline(always)]
    fn retrieve_outcome(outcome: &ExpectationOutcome) -> &Self::Outcome {
        match outcome {
            ExpectationOutcome::NumType(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FlyTerms) -> FinalDestination {
        todo!()
    }

    #[inline(always)]
    fn destination(&self) -> Option<FlyTerm> {
        None
    }

    fn resolve(
        &self,
        db: &::salsa::Db,
        fluffy_terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        let expectee = state.expectee();
        match expectee.base_ty_data_inner(db, fluffy_terms) {
            FlyBaseTypeData::TypeOntology {
                refined_ty_path: Left(PreludeTypePath::Num(PreludeNumTypePath::Int(_))),
                ..
            } => state.set_ok(
                ExpectNumTypeOutcome {
                    placeless_num_ty: state.expectee(),
                },
                smallvec![],
            ),
            FlyBaseTypeData::Hole(hole_kind, hole) => match hole_kind {
                HoleKind::UnspecifiedIntegerType | HoleKind::UnspecifiedFloatType => state.set_ok(
                    ExpectNumTypeOutcome {
                        placeless_num_ty: hole.into(),
                    },
                    smallvec![],
                ),
                HoleKind::ImplicitType | HoleKind::Any => AltNone,
            },
            _ => state.set_err(
                OriginalFlyTermExpectationError::ExpectedIntType { expectee },
                smallvec![],
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExpectNumTypeOutcome {
    /// guaranteed to be placeless
    placeless_num_ty: FlyTerm,
}

impl ExpectNumTypeOutcome {
    /// guaranteed to be placeless
    pub fn placeless_num_ty(&self) -> FlyTerm {
        self.placeless_num_ty
    }
}
