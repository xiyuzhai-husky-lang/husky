use super::*;

impl HasFluffyTraitMethodDispatch for HollowTerm {
    fn trai_method_dispatch_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        trai_item_records: TraitInUseItemsWithGivenIdent,
        indirections: FluffyIndirections,
    ) -> FluffyTermMaybeResult<FluffyMethodDynamicDispatch> {
        todo!()
    }
}
