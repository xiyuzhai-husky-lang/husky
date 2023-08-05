use super::*;

impl ExpectCoersion {
    pub(super) fn resolve_wrap_in_some(
        &self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<ExpectationEffect> {
        let (expectee_place, expectee_base_ty_data) = state.expectee().ty_data_inner(db, terms);
        let (expected_place, expected_base_ty_data) = self.ty_expected.ty_data_inner(db, terms);
        // todo: check contract
        match expected_base_ty_data {
            FluffyBaseTypeData::TypeOntology {
                refined_ty_path: Left(PreludeTypePath::Option),
                ty_arguments: expected_ty_arguments,
                ..
            } => {
                debug_assert_eq!(expected_ty_arguments.len(), 1);
                let (expected_ty_argument_place, expected_ty_argument_base_ty_data) =
                    expected_ty_arguments[0].ty_data_inner(db, terms);
                // todo: check expected_ty_argument_place
                resolve_aux(
                    expectee_base_ty_data,
                    expected_ty_argument_base_ty_data,
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
//     if let Left(PreludeTypePath::Option) = dst_refined_ty_path && dst_ty_arguments[0] == state.expectee() {
//         debug_assert_eq!(dst_ty_arguments.len() ,1);
//         state.set_ok(Coersion::WrapInSome, smallvec![])
//     } else{
