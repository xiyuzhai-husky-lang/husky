use super::*;

/// expect primitive number types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectNumType;

impl ExpectFluffyTerm for ExpectNumType {
    type Outcome = ExpectNumTypeOutcome;

    #[inline(always)]
    fn retrieve_outcome(outcome: &FluffyTermExpectationOutcome) -> &Self::Outcome {
        match outcome {
            FluffyTermExpectationOutcome::NumType(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(
        &self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> FinalDestination {
        todo!()
    }

    #[inline(always)]
    fn destination(&self) -> Option<FluffyTerm> {
        None
    }

    fn resolve(
        &self,
        db: &dyn FluffyTermDb,
        state: &mut ExpectationMeta,
        fluffy_terms: &mut FluffyTerms,
    ) -> Option<ExpectationEffect> {
        match state.expectee().data_inner(db, fluffy_terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                arguments,
                ..
            } => todo!(),
            FluffyTermData::PlaceTypeOntology {
                place,
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
            } => todo!(),
            FluffyTermData::Hole(hole_kind, _) => match hole_kind {
                HoleKind::UnspecifiedIntegerType | HoleKind::UnspecifiedFloatType => None,
                HoleKind::ImplicitType => todo!(),
            },
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => match hole_kind {
                HoleKind::UnspecifiedIntegerType | HoleKind::UnspecifiedFloatType => state.set_ok(
                    ExpectNumTypeOutcome {
                        placeless_num_ty: hole.into(),
                    },
                    smallvec![],
                ),
                HoleKind::ImplicitType => todo!(),
            },
            FluffyTermData::Symbol { ty } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExpectNumTypeOutcome {
    /// guaranteed to be placeless
    placeless_num_ty: FluffyTerm,
}

impl ExpectNumTypeOutcome {
    /// guaranteed to be placeless
    pub fn placeless_num_ty(&self) -> FluffyTerm {
        self.placeless_num_ty
    }
}
