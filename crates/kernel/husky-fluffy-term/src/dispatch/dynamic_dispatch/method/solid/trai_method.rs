use super::*;

impl HasFluffyTraitMethodDispatch for SolidTerm {
    fn trai_method_dispatch_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: ExprIdx,
        ident_token: IdentToken,
        trai_item_records: &[TraitInUseItemRecord],
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
        todo!()
    }
}
