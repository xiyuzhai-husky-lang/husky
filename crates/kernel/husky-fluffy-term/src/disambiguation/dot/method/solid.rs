use super::*;

impl SolidTerm {
    pub(super) fn method_disambiguation_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: ExprIdx,
        ident: Ident,
        available_traits: &[TraitPath],
        mut indirections: SmallVec<[FluffyDotIndirection; 2]>,
    ) -> FluffyTermMaybeResult<FluffyMethodDisambiguation> {
        match self.data(engine) {
            SolidTermData::TypeOntology {
                path,
                refined_path,
                arguments,
            } => todo!(),
            SolidTermData::TypeOntologyAtPlace {
                place,
                path,
                refined_path,
                arguments,
                base_ty_term,
            } => match base_ty_term {
                Some(base_ty_term) => {
                    indirections.push(FluffyDotIndirection::Place(*place));
                    JustOk(
                        ethereal_ty_method_disambiguation(engine, expr_idx, *base_ty_term, ident)?
                            .merge(indirections),
                    )
                }
                None => todo!(),
            },
            SolidTermData::Curry { .. } | SolidTermData::Ritchie { .. } => Nothing,
            SolidTermData::SymbolAtPlace { .. } => todo!(),
        }
    }
}
