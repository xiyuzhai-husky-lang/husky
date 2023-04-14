use super::*;

/// expect primitive number types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectNumType;

impl ExpectNumType {
    #[inline(always)]
    pub(super) fn resolve(
        self,
        db: &dyn FluffyTermDb,
        fluffy_terms: &mut FluffyTerms,
        expectee: FluffyTerm,
    ) -> Option<FluffyTermExpectationEffect> {
        match expectee.data_inner(db, fluffy_terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                path,
                refined_path,
                arguments,
            } => todo!(),
            FluffyTermData::PlaceTypeOntology {
                place,
                path,
                refined_path,
                arguments,
            } => todo!(),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
            } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => match hole_kind {
                HoleKind::UnspecifiedIntegerType | HoleKind::UnspecifiedFloatType => {
                    Some(FluffyTermExpectationEffect {
                        result: Ok(ExpectNumTypeOutcome {
                            placeless_num_ty: hole.into(),
                        }
                        .into()),
                        actions: smallvec![],
                    })
                }
                HoleKind::ImplicitType => todo!(),
            },
        }
    }
}

impl ExpectFluffyTerm for ExpectNumType {
    type Outcome = ExpectNumTypeOutcome;

    fn retrieve_outcome(outcome: &FluffyTermExpectationOutcome) -> &Self::Outcome {
        match outcome {
            FluffyTermExpectationOutcome::NumType(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    fn final_destination_inner(
        &self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> FinalDestination {
        todo!()
    }

    fn destination(&self) -> Option<FluffyTerm> {
        todo!()
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
