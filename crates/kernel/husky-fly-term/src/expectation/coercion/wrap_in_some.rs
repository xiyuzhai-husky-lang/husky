use super::*;
use husky_entity_path::path::major_item::ty::PreludeTypePath;

impl ExpectCoercion {
    pub(super) fn resolve_wrap_in_some(
        &self,
        db: &::salsa::Db,
        terms: &FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        let expected_base_ty_data = self.ty_expected.base_ty_data_inner(db, terms);
        match expected_base_ty_data {
            FlyBaseTypeData::TypeOntology {
                refined_ty_path: Left(PreludeTypePath::Option),
                ty_arguments: expected_ty_arguments,
                ..
            } => {
                debug_assert_eq!(expected_ty_arguments.len(), 1);
                self.try_finalize_coercion(
                    state.expectee(),
                    expected_ty_arguments[0],
                    FlyCoercion::WrapInSome,
                    db,
                    terms,
                    state,
                )
            }
            _ => AltNone,
        }
    }
}
