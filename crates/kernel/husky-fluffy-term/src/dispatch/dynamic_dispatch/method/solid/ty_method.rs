use super::*;

impl HasFluffyTypeMethodDispatch for SolidTerm {
    fn ty_method_dispatch(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        indirections: FluffyDynamicDispatchIndirections,
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
        match self.data(engine) {
            SolidTermData::TypeOntology {
                path,
                refined_path,
                arguments,
            } => todo!(),
            // SolidTermData::TypeOntologyAtPlace {
            //     place,
            //     ty_path: path,
            //     refined_ty_path: refined_path,
            //     arguments,
            //     base_ty_term,
            // } => match base_ty_term.as_ref() {
            //     Some(&base_ty_term) => {
            //         indirections.push(FluffyDynamicDispatchIndirection::Place(*place));
            //         JustOk(
            //             base_ty_term
            //                 .ty_method_dispatch(engine, expr_idx, ident_token)?
            //                 .merge(indirections),
            //         )
            //     }
            //     None => todo!(),
            // },
            SolidTermData::Curry { .. } | SolidTermData::Ritchie { .. } => Nothing,
        }
    }
}
