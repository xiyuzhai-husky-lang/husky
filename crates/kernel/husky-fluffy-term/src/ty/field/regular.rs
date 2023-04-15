use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct FluffyRegularFieldDisambiguation {
    indirections: SmallVec<[FluffyFieldIndirection; 2]>,
    ty_path: TypePath,
}

impl FluffyTerm {
    pub(super) fn regular_field_ty(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
    ) -> FluffyTypeResult<
        Option<(
            FluffyRegularFieldDisambiguation,
            FluffyTypeResult<FluffyTerm>,
        )>,
    > {
        match self.nested() {
            NestedFluffyTerm::Ethereal(term) => {
                todo!()
                // let Some(card) = term.regular_field_ty(engine.db(), ident)? else {
                //     return Ok(None)
                // };
                // Ok(Some((todo!(), todo!())))
            }
            NestedFluffyTerm::Solid(term) => term.regular_field_ty(engine, ident),
            NestedFluffyTerm::Hollow(term) => term.regular_field_ty(engine, ident),
        }
    }
}

impl SolidTerm {
    fn regular_field_ty(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
    ) -> FluffyTypeResult<
        Option<(
            FluffyRegularFieldDisambiguation,
            FluffyTypeResult<FluffyTerm>,
        )>,
    > {
        let mut indirections = smallvec![];
        let Some((ty_path, ty_result)) = self.regular_field_ty_aux(engine,ident,&mut indirections)? else {
            return Ok(None)
        };
        Ok(Some((
            FluffyRegularFieldDisambiguation {
                indirections,
                ty_path,
            },
            ty_result,
        )))
    }

    fn regular_field_ty_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        indirections: &mut SmallVec<[FluffyFieldIndirection; 2]>,
    ) -> FluffyTypeResult<Option<(TypePath, FluffyTypeResult<FluffyTerm>)>> {
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
                    todo!();
                    // let Some(card) = base_ty_term.regular_field_card(db, ident)? else {
                    //     return Ok(None)
                    // };
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

impl HollowTerm {
    fn regular_field_ty(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
    ) -> FluffyTypeResult<
        Option<(
            FluffyRegularFieldDisambiguation,
            FluffyTypeResult<FluffyTerm>,
        )>,
    > {
        todo!()
    }
}
