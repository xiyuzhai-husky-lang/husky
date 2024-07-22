use super::*;

impl SolTerm {
    pub(super) fn disambiguate_index(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        index_ty: FlyTerm,
    ) -> FlyTermMaybeResult<FlyIndexInstanceDispatch> {
        self.disambiguate_index_aux(engine, expr_idx, index_ty, FlyIndirections::new(todo!()))
    }

    pub(super) fn disambiguate_index_aux(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        index_ty: FlyTerm,
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<FlyIndexInstanceDispatch> {
        let db = engine.db();
        match self.data(engine) {
            SolTermData::TypeOntology {
                path,
                refined_path,
                arguments,
            } => todo!(),
            SolTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_hvar,
                parameter_ty,
                return_ty,
            } => todo!(),
            SolTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
        }
    }
}
