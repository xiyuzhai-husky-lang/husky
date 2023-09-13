use super::*;

impl HasFluffyTypeMethodDispatch for HollowTerm {
    fn ty_method_dispatch(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
        match self.fluffy_base_ty_data(engine.db(), engine.fluffy_terms()) {
            FluffyBaseTypeData::TypeOntology {
                ty_path,
                refined_ty_path,
                ty_arguments,
                ty_ethereal_term,
            } => todo!(),
            FluffyBaseTypeData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FluffyBaseTypeData::Hole(hole_kind, hole) => {
                let db = engine.db();
                let term_menu = engine.term_menu();
                engine
                    .fluffy_term_region_mut()
                    .terms
                    .fill_hole_by_force(hole, db, term_menu);
                // ad hoc, needs improvement
                Into::<FluffyTerm>::into(self).ty_method_dispatch(engine, expr_idx, ident_token)
            }
            FluffyBaseTypeData::Category(_) => todo!(),
            FluffyBaseTypeData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FluffyBaseTypeData::Symbol { term } => todo!(),
        }
    }
}
