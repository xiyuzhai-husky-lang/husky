use super::*;

impl SolidTerm {
    pub(super) fn field_dispatch_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
        mut indirections: FluffyIndirections,
    ) -> FluffyTermMaybeResult<FluffyFieldDyanmicDispatch> {
        match self.data(engine) {
            SolidTermData::TypeOntology {
                path,
                refined_path,
                arguments,
            } => todo!(),
            SolidTermData::Curry { .. } | SolidTermData::Ritchie { .. } => Nothing,
        }
    }
}
