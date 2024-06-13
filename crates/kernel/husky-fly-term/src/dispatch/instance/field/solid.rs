use super::*;

impl SolTerm {
    pub(super) fn field_dispatch_aux(
        self,
        engine: &mut impl FlyTermEngineMut,
        ident: Ident,
        available_traits: &[TraitPath],
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<FlyFieldInstanceDispatch> {
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
