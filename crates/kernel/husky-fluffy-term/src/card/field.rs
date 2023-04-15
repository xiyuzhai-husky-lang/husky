mod leashed;
mod memo;
mod regular;

use super::*;
use husky_word::Ident;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FluffyFieldCard {
    place: Option<Place>,
    visibility: Visibility,
    modifier: FieldModifier,
    ty: FluffyTerm,
}

impl FluffyFieldCard {
    fn from_ethereal(place: Option<Place>, field_card: RegularFieldCard) -> Self {
        Self {
            place,
            visibility: field_card.visibility(),
            modifier: field_card.modifier(),
            ty: field_card.ty().into(),
        }
    }

    fn to_term(self, engine: &mut impl FluffyTermEngine) -> FluffyTerm {
        todo!()
    }
}

impl FluffyTerm {
    pub fn field_card(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
    ) -> FluffyCardResult<Option<FluffyFieldCard>> {
        match self.nested() {
            NestedFluffyTerm::Ethereal(_) => todo!(),
            NestedFluffyTerm::Solid(term) => term.field_card(engine, ident),
            NestedFluffyTerm::Hollow(_) => todo!(),
        }
        // Ok(self
        //     .field_card_aux(engine, ident)?
        //     .map(|field_ty| field_ty.to_term(engine)))
    }

    // fn field_card_aux(
    //     self,
    //     engine: &mut impl FluffyTermEngine,
    //     ident: Ident,
    // ) -> FluffyCardResult<Option<FluffyFieldCard>> {
    //     // match self {
    //     //     FluffyTerm::Literal(_) => todo!(),
    //     //     FluffyTerm::Symbol(_) => todo!(),
    //     //     FluffyTerm::Hole(_) => todo!(),
    //     //     FluffyTerm::EntityPath(_) => todo!(),
    //     //     FluffyTerm::Category(_) => todo!(),
    //     //     FluffyTerm::Universe(_) => todo!(),
    //     //     FluffyTerm::Curry(_) => todo!(),
    //     //     FluffyTerm::Ritchie(_) => todo!(),
    //     //     FluffyTerm::Abstraction(_) => todo!(),
    //     //     FluffyTerm::Application(_) => todo!(),
    //     //     FluffyTerm::Subentity(_) => todo!(),
    //     //     FluffyTerm::AsTraitSubentity(_) => todo!(),
    //     //     FluffyTerm::TraitConstraint(_) => todo!(),
    //     //     FluffyTerm::Solid(term) => term
    //     //         .field_card(engine, ident)
    //     //         .map(|opt| opt.map(Into::into)),
    //     //     FluffyTerm::Hollow(_) => todo!(),
    //     // }
    //     // let owner_ty_unravelled =
    //     //     owner_ty.unravel_borrow(self.db, self.fluffy_term_region.porous_terms());
    //     // match owner_ty_unravelled {
    //     //     FluffyTerm::Term(owner_ty_unravelled) => {
    //     //         match self.db.field_ty(owner_ty_unravelled, ident_token.ident()) {
    //     //             Ok(Some(field_ty)) => Ok(field_ty.into()),
    //     //             Ok(None) => Err(OriginalExprTypeError::NoSuchField.into()),
    //     //             Err(e) => Err(DerivedExprTypeError::FieldTypeTermError(e).into()),
    //     //         }
    //     //     }
    //     //     FluffyTerm::Unresolved(_) => todo!(),
    //     //     _ => todo!(),
    //     // }
    // }
}

impl SolidTerm {
    fn field_card(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
    ) -> FluffyCardResult<Option<FluffyFieldCard>> {
        match self.data(engine) {
            SolidTermData::TypeOntology {
                path,
                refined_path,
                arguments: argument_tys,
            } => todo!(),
            SolidTermData::PlaceTypeOntology {
                place,
                base_ty_term: Some(base_ty_term),
                ..
            } => Ok(base_ty_term
                .regular_field_card(engine.db(), ident)?
                .map(|field_card| FluffyFieldCard::from_ethereal(Some(*place), field_card))),
            SolidTermData::PlaceTypeOntology {
                place,
                path,
                refined_path,
                arguments: argument_tys,
                ..
            } => todo!(),
            SolidTermData::Curry { .. } | SolidTermData::Ritchie { .. } => todo!(),
        }
    }
}
