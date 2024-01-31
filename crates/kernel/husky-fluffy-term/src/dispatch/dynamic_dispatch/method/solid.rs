mod trai_method;
mod ty_method;

use super::*;

impl SolidTerm {
    pub(super) fn method_dispatch_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident: Ident,
        available_traits: &[TraitPath],
        mut indirections: FluffyIndirections,
    ) -> FluffyTermMaybeResult<FluffyMethodDynamicDispatch> {
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
