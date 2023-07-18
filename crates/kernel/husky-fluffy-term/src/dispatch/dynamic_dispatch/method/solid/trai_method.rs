use super::*;

impl HasFluffyTraitMethodDispatch for SolidTerm {
    fn trai_method_dispatch_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: ExprIdx,
        ident_token: IdentToken,
        trai_item_records: TraitInUseItemsWithGivenIdent,
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
        match self.data(engine) {
            SolidTermData::TypeOntology {
                path,
                refined_path,
                arguments,
            } => todo!(),
            SolidTermData::TypeOntologyAtPlace {
                path,
                refined_path,
                arguments,
                base_ty_term,
                place,
            } => match base_ty_term {
                Some(_) => todo!(),
                None => todo!(),
            },
            SolidTermData::SymbolAtPlace { term, place } => todo!(),
            SolidTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
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
