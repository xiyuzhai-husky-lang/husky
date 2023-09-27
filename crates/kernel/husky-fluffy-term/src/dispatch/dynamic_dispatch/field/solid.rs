use super::*;

impl SolidTerm {
    pub(super) fn field_dispatch_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
        mut indirections: FluffyTermDynamicDispatchIndirections,
    ) -> FluffyTermMaybeResult<FluffyFieldDyanmicDispatch> {
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
            // } => match base_ty_term {
            //     Some(base_ty_term) => {
            //         indirections.push(FluffyTermDynamicDispatchIndirection::Place(*place));
            //         JustOk(
            //             ethereal_ty_field_dispatch(engine.db(), *base_ty_term, ident)?
            //                 .merge(indirections),
            //         )
            //     }
            //     None => todo!(),
            // },
            SolidTermData::Curry { .. } | SolidTermData::Ritchie { .. } => Nothing,
        }
    }
}
