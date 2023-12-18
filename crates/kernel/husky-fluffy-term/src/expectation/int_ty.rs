use super::*;

/// expect primitive number types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectIntType;

impl ExpectFluffyTerm for ExpectIntType {
    type Outcome = ExpectIntTypeOutcome;

    #[inline(always)]
    fn retrieve_outcome(outcome: &ExpectationOutcome) -> &Self::Outcome {
        match outcome {
            ExpectationOutcome::IntType(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FluffyTerms) -> FinalDestination {
        todo!()
    }

    #[inline(always)]
    fn destination(&self) -> Option<FluffyTerm> {
        None
    }

    fn resolve(
        &self,
        db: &::salsa::Db,
        fluffy_terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FluffyTermEffect> {
        match state.expectee().data_inner(db, fluffy_terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
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
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_rune: parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Hole(hole_kind, _) => match hole_kind {
                HoleKind::UnspecifiedIntegerType => AltNone,
                HoleKind::UnspecifiedFloatType => todo!(),
                HoleKind::ImplicitType => todo!(),
                HoleKind::Any => todo!(),
            },
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => todo!(),
            // FluffyTermData::HoleAtPlace {
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
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::Rune { .. } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExpectIntTypeOutcome {
    /// guaranteed to be placeless
    placeless_num_ty: FluffyTerm,
}

impl ExpectIntTypeOutcome {
    /// guaranteed to be placeless
    pub fn placeless_num_ty(&self) -> FluffyTerm {
        self.placeless_num_ty
    }
}
