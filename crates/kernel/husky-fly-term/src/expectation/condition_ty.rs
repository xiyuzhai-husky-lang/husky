use super::*;
use husky_entity_path::path::major_item::ty::{
    PreludeBasicTypePath, PreludeNumTypePath, PreludeTypePath,
};
use husky_expr::stmt::ConditionConversion;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectConditionType;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectConditionTypeOutcome {
    pub conversion: ConditionConversion,
}

impl ExpectFlyTerm for ExpectConditionType {
    type Outcome = ExpectConditionTypeOutcome;

    fn retrieve_outcome(outcome: &ExpectationOutcome) -> &Self::Outcome {
        match outcome {
            ExpectationOutcome::ConditionType(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FlyTerms) -> FinalDestination {
        FinalDestination::TypeOntology
    }

    fn destination(&self) -> FlyTermDestination {
        todo!()
    }

    fn resolve(
        &self,
        db: &::salsa::Db,
        terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        match state.expectee().data_inner(db, terms) {
            FlyTermData::TypeOntology {
                refined_ty_path, ..
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
            _ => todo!(),
        }
    }
}
