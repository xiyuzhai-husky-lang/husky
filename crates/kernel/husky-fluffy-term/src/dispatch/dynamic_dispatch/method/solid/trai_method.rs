use super::*;

impl HasFluffyTraitMethodDispatch for SolidTerm {
    fn trai_method_dispatch_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        trai_item_records: TraitInUseItemsWithGivenIdent,
        indirections: FluffyDynamicDispatchIndirections,
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
        match self.data(engine) {
            SolidTermData::TypeOntology {
                path,
                refined_path,
                arguments,
            } => todo!(),
            // SolidTermData::TypeOntologyAtPlace {
            //     ty_path: path,
            //     refined_ty_path: refined_path,
            //     arguments,
            //     base_ty_term,
            //     place,
            // } => match base_ty_term.as_ref() {
            //     Some(&base_ty_term) => {
            //         indirections.push(FluffyDynamicDispatchIndirection::Place(*place));
            //         JustOk(
            //             base_ty_term
            //                 .trai_method_dispatch_aux(
            //                     engine,
            //                     expr_idx,
            //                     ident_token,
            //                     trai_item_records,
            //                 )?
            //                 .merge(indirections),
            //         )
            //     }
            //     None => todo!(),
            // },
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
