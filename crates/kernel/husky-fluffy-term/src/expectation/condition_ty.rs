use super::*;
use husky_expr::stmt::ConditionConversion;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectConditionType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectConditionTypeOutcome {
    pub conversion: ConditionConversion,
}

impl ExpectFluffyTerm for ExpectConditionType {
    type Outcome = ExpectConditionTypeOutcome;

    fn retrieve_outcome(outcome: &ExpectationOutcome) -> &Self::Outcome {
        match outcome {
            ExpectationOutcome::ConditionType(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FluffyTerms) -> FinalDestination {
        FinalDestination::TypeOntology
    }

    fn destination(&self) -> Option<FluffyTerm> {
        todo!()
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
                ty_path,
                refined_ty_path,
                ty_arguments,
                ty_ethereal_term,
            } => match refined_ty_path {
                Left(PreludeTypePath::Num(PreludeNumTypePath::Int(prelude_ty_path))) => state
                    .set_ok(
                        ExpectConditionTypeOutcome {
                            conversion: ConditionConversion::IntToBool(prelude_ty_path),
                        },
                        smallvec![],
                    ),
                Left(PreludeTypePath::Basic(PreludeBasicTypePath::Bool)) => state.set_ok(
                    ExpectConditionTypeOutcome {
                        conversion: ConditionConversion::None,
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
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FluffyTermData::Symbol { term, ty } => todo!(),
            FluffyTermData::Rune { ty } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}
