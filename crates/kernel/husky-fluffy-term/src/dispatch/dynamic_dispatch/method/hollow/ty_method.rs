use super::*;

impl HasFluffyTypeMethodDispatch for HollowTerm {
    fn ty_method_dispatch(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentToken,
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
        todo!()
    }
}
