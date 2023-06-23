mod ethereal;
mod hollow;
mod solid;

use self::ethereal::*;
use self::hollow::*;
use self::solid::*;
use super::*;
use husky_word::Ident;

pub enum AssociatedItemDisambiguation {
    AssociatedFn,
}

impl FluffyTerm {
    pub fn disambiguate_associated_item(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        all_available_traits: &[()],
    ) -> AssociatedItemDisambiguation {
        match self.data(engine) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path,
                refined_ty_path,
                arguments,
                ty_ethereal_term,
            } => {
                let signature_templates =
                    ty_path.associated_item_signature_templates(ident, all_available_traits);
                todo!()
            }
            FluffyTermData::PlaceTypeOntology {
                place,
                ty_path,
                refined_ty_path,
                arguments,
                base_ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
            FluffyTermData::Symbol { ty } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
        }
    }
}
