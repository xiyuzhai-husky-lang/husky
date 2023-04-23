use super::*;

impl SolidTerm {
    pub(super) fn field_disambiguation_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
        mut indirections: SmallVec<[FluffyFieldIndirection; 2]>,
    ) -> FluffyTermResult<Option<FluffyFieldDisambiguation>> {
        match self.data(engine) {
            SolidTermData::TypeOntology {
                path,
                refined_path,
                arguments,
            } => todo!(),
            SolidTermData::PlaceTypeOntology {
                place,
                path,
                refined_path,
                arguments,
                base_ty_term,
            } => match base_ty_term {
                Some(base_ty_term) => {
                    let db = engine.db();
                    let Some(disambiguation) = ethereal_ty_field_disambiguation(
                        db,
                        *base_ty_term,
                        ident
                    )? else {
                        return Ok(None)
                    };
                    indirections.push(FluffyFieldIndirection::Place(*place));
                    indirections.extend(disambiguation.indirections.iter().copied());
                    todo!()
                    // match place {
                    //     Place::Const => todo!(),
                    //     Place::StackPure { location } => todo!(),
                    //     Place::ImmutableStackOwned { location } => todo!(),
                    //     Place::MutableStackOwned { location } => todo!(),
                    //     Place::Transient => todo!(),
                    //     Place::Ref { guard } => todo!(),
                    //     Place::RefMut { guard } => todo!(),
                    //     Place::Leashed => todo!(),
                    //     Place::Todo => todo!(),
                    // }
                    // Ok(Some((todo!(), todo!())))
                }
                None => todo!(),
            },
            SolidTermData::Curry { .. } | SolidTermData::Ritchie { .. } => Ok(None),
        }
    }
}
