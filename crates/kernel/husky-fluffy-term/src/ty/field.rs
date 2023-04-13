use super::*;
use husky_word::Ident;

impl FluffyTerm {
    pub fn field_ty(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
    ) -> FluffyTypeResult<Self> {
        match self {
            FluffyTerm::Literal(_) => todo!(),
            FluffyTerm::Symbol(_) => todo!(),
            FluffyTerm::Hole(_) => todo!(),
            FluffyTerm::EntityPath(_) => todo!(),
            FluffyTerm::Category(_) => todo!(),
            FluffyTerm::Universe(_) => todo!(),
            FluffyTerm::Curry(_) => todo!(),
            FluffyTerm::Ritchie(_) => todo!(),
            FluffyTerm::Abstraction(_) => todo!(),
            FluffyTerm::Application(_) => todo!(),
            FluffyTerm::Subentity(_) => todo!(),
            FluffyTerm::AsTraitSubentity(_) => todo!(),
            FluffyTerm::TraitConstraint(_) => todo!(),
            FluffyTerm::Solid(term) => term.field_ty(engine, ident).map(Into::into),
            FluffyTerm::Hollow(_) => todo!(),
        }
        // let owner_ty_unravelled =
        //     owner_ty.unravel_borrow(self.db, self.fluffy_term_region.porous_terms());
        // match owner_ty_unravelled {
        //     FluffyTerm::Term(owner_ty_unravelled) => {
        //         match self.db.field_ty(owner_ty_unravelled, ident_token.ident()) {
        //             Ok(Some(field_ty)) => Ok(field_ty.into()),
        //             Ok(None) => Err(OriginalExprTypeError::NoSuchField.into()),
        //             Err(e) => Err(DerivedExprTypeError::FieldTypeTermError(e).into()),
        //         }
        //     }
        //     FluffyTerm::Unresolved(_) => todo!(),
        //     _ => todo!(),
        // }
    }
}

impl SolidTerm {
    fn field_ty(self, engine: &mut impl FluffyTermEngine, ident: Ident) -> FluffyTypeResult<Self> {
        match self.data(engine.fluffy_terms().solid_terms()) {
            SolidTermData::TypeOntology {
                path,
                refined_path,
                argument_tys,
            } => todo!(),
            SolidTermData::PlaceTypeOntology {
                place,
                path,
                refined_path,
                argument_tys,
                base_ty_term,
            } => match base_ty_term {
                Some(_) => todo!(),
                None => todo!(),
            },
            SolidTermData::Curry { .. } | SolidTermData::Ritchie { .. } => todo!(),
        }
    }
}
