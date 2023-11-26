use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DerefCoersion {
    Leash,
    Ref,
}

impl ExpectCoersion {
    pub(super) fn resolve_deref(
        &self,
        db: &::salsa::Db,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FluffyTermEffect> {
        let expectee_base_ty_data = state.expectee().base_ty_data_inner(db, terms);
        // todo: check contract
        match expectee_base_ty_data {
            FluffyBaseTypeData::TypeOntology {
                refined_ty_path: Left(PreludeTypePath::Indirection(prelude_indirection_ty_path)),
                ty_arguments: expectee_ty_arguments,
                ..
            } => {
                match prelude_indirection_ty_path {
                    PreludeIndirectionTypePath::Ref => {
                        debug_assert_eq!(expectee_ty_arguments.len(), 2);
                        // todo: check expected_ty_argument_place
                        resolve_aux(
                            self.ty_expected,
                            expectee_ty_arguments[1],
                            |_, _| Some(Coersion::Deref(DerefCoersion::Ref)),
                            db,
                            terms,
                            state,
                        )
                    }
                    PreludeIndirectionTypePath::RefMut => todo!(),
                    PreludeIndirectionTypePath::Leash => {
                        debug_assert_eq!(expectee_ty_arguments.len(), 1);
                        // todo: check expected_ty_argument_place
                        resolve_aux(
                            self.ty_expected,
                            expectee_ty_arguments[0],
                            |_, _| Some(Coersion::Deref(DerefCoersion::Leash)),
                            db,
                            terms,
                            state,
                        )
                    }
                    PreludeIndirectionTypePath::At => todo!(),
                }
            }
            _ => AltNone,
        }
    }
}
