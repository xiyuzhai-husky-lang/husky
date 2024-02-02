use super::*;

impl SolTerm {
    pub(super) fn field_dispatch_aux(
        self,
        engine: &mut impl FlyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
        mut indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<FlyFieldDyanmicDispatch> {
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
