use super::*;

impl HasFlyTypeMethodDispatch for HolTerm {
    fn ty_method_dispatch(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<FlyMethodDynamicDispatch> {
        match self.fluffy_base_ty_data(engine.db(), engine.fluffy_terms()) {
            FlyBaseTypeData::TypeOntology {
                ty_path,
                refined_ty_path,
                ty_arguments,
                ty_ethereal_term,
            } => {
                p!(self
                    .fluffy_base_ty_data(engine.db(), engine.fluffy_terms())
                    .debug(engine.db()));
                todo!()
            }
            FlyBaseTypeData::Curry {
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FlyBaseTypeData::Hole(hole_kind, hole) => {
                let db = engine.db();
                let term_menu = engine.term_menu();
                engine
                    .fluffy_term_region_mut()
                    .terms
                    .fill_hole_by_force(hole, db, term_menu);
                // ad hoc, needs improvement
                Into::<FlyTerm>::into(self).ty_method_dispatch(
                    engine,
                    expr_idx,
                    ident_token,
                    indirections,
                )
            }
            FlyBaseTypeData::Category(_) => todo!(),
            FlyBaseTypeData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FlyBaseTypeData::Symbol { symbol: term } => todo!(),
            FlyBaseTypeData::Rune { rune } => todo!(),
        }
    }
}
