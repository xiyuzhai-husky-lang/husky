use super::*;

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
    fn destination(&self) -> Option<FlyTerm> {
        None
    }

    fn resolve(
        &self,
        db: &::salsa::Db,
        fluffy_terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        match state.expectee().data_inner(db, fluffy_terms) {
            FlyTermData::Literal(_) => todo!(),
            FlyTermData::TypeOntology {
                ty_path,
                refined_ty_path,
                ty_arguments,
                ..
            } => match refined_ty_path {
                Left(PreludeTypePath::Num(_)) => state.set_ok(
                    ExpectIntTypeOutcome {
                        placeless_num_ty: state.expectee(),
                    },
                    smallvec![],
                ),
                _ => todo!(),
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
            FlyTermData::Hole(hole_kind, _) => match hole_kind {
                HoleKind::UnspecifiedIntegerType => AltNone,
                HoleKind::UnspecifiedFloatType => todo!(),
                HoleKind::ImplicitType => todo!(),
                HoleKind::Any => todo!(),
            },
            FlyTermData::Category(_) => todo!(),
            FlyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => todo!(),
            // FlyTermData::HoleAtPlace {
            //     place,
            //     hole_kind,
            //     hole,
            // } => match hole_kind {
            //     HoleKind::UnspecifiedIntegerType | HoleKind::UnspecifiedFloatType => state.set_ok(
            //         ExpectNumTypeOutcome {
            //             placeless_num_ty: hole.into(),
            //         },
            //         smallvec![],
            //     ),
            //     HoleKind::ImplicitType => todo!(),
            //     HoleKind::Any => todo!(),
            // },
            FlyTermData::Symbol { .. } => todo!(),
            FlyTermData::Rune { .. } => todo!(),
            FlyTermData::TypeVariant { path } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExpectIntTypeOutcome {
    /// guaranteed to be placeless
    placeless_num_ty: FlyTerm,
}

impl ExpectIntTypeOutcome {
    /// guaranteed to be placeless
    pub fn placeless_num_ty(&self) -> FlyTerm {
        self.placeless_num_ty
    }
}
