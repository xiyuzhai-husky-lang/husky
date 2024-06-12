use super::*;

impl HasFlyTraitMethodDispatch for HolTerm {
    fn trai_method_dispatch_aux(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        trai_item_records: AvailableTraitItemsWithGivenIdent,
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<FlyMethodDynamicDispatch> {
        todo!()
    }
}
