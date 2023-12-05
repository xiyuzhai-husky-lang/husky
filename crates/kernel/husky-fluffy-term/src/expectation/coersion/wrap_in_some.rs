use super::*;

impl ExpectCoersion {
    pub(super) fn resolve_wrap_in_some(
        &self,
        db: &::salsa::Db,
        terms: &FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FluffyTermEffect> {
        let expected_base_ty_data = self.ty_expected.base_ty_data_inner(db, terms);
        // todo: check contract
        match expected_base_ty_data {
            FluffyBaseTypeData::TypeOntology {
                refined_ty_path: Left(PreludeTypePath::Option),
                ty_arguments: expected_ty_arguments,
                ..
            } => {
                debug_assert_eq!(expected_ty_arguments.len(), 1);
                // todo: check expected_ty_argument_place
                try_finalize_coersion(
                    state.expectee(),
                    expected_ty_arguments[0],
                    Coersion::WrapInSome,
                    db,
                    terms,
                    state,
                )
            }
            _ => AltNone,
        }
    }
}
