use super::*;

impl HasFlyTraitMethodDispatch for SolTerm {
    fn trai_method_dispatch_aux(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        trai_item_records: AvailableTraitItemsWithGivenIdent,
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<FlyMethodDynamicDispatch> {
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
                parameter_hvar,
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
