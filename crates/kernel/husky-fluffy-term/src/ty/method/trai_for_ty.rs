use super::*;

impl FluffyTerm {
    pub(super) fn trai_for_ty_method_ty(
        self,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> FluffyMethodTypeResult<
        Option<(
            SmallVec<[FluffyMethodIndirection; 2]>,
            TypePath,
            TraitPath,
            FluffyTerm, // trai
            FluffyMethodTypeResult<FluffyTerm>,
        )>,
    > {
        for trai in available_traits {
            todo!()
        }
        Ok(None)
    }
}
