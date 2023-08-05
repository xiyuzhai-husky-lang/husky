use super::*;

impl ExpectCoersion {
    pub(super) fn resolve_wrap_in_some(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<ExpectationEffect> {
        // ad hoc
        AltNone
    }
}
//     if let Left(PreludeTypePath::Option) = dst_refined_ty_path && dst_ty_arguments[0] == state.expectee() {
//         debug_assert_eq!(dst_ty_arguments.len() ,1);
//         state.set_ok(Coersion::WrapInSome, smallvec![])
//     } else{
