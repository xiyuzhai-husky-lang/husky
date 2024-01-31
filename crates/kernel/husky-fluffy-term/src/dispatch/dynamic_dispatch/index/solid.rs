use super::*;

impl SolidTerm {
    pub(super) fn disambiguate_index(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        index_ty: FluffyTerm,
    ) -> FluffyTermMaybeResult<FluffyIndexDynamicDispatch> {
        self.disambiguate_index_aux(engine, expr_idx, index_ty, FluffyIndirections::new(todo!()))
    }

    pub(super) fn disambiguate_index_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        index_ty: FluffyTerm,
        mut indirections: FluffyIndirections,
    ) -> FluffyTermMaybeResult<FluffyIndexDynamicDispatch> {
        let db = engine.db();
        match self.data(engine) {
            SolidTermData::TypeOntology {
                path,
                refined_path,
                arguments,
            } => todo!(),
            SolidTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
            } => todo!(),
            SolidTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
        }
    }
}
