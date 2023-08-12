use super::*;

impl HasFluffyTypeMethodDispatch for HollowTerm {
    fn ty_method_dispatch(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentToken,
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
        p!(
            engine.expr_region_data().path().debug(engine.db()),
            self.fluffy_data(engine.db(), engine.fluffy_terms())
                .show(engine.db(), engine.fluffy_terms())
        );
        todo!()
    }
}
