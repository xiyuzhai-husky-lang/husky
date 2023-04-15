use super::*;

impl FluffyTerm {
    pub(super) fn ty_method_ty(
        self,
        ident: Ident,
    ) -> FluffyMethodTypeResult<
        Option<(
            SmallVec<[FluffyMethodIndirection; 2]>,
            TypePath,
            FluffyMethodTypeResult<FluffyTerm>,
        )>,
    > {
        todo!()
    }
}
