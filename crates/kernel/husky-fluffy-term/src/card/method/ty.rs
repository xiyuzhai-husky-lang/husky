use super::*;
use husky_entity_tree::AssociatedItemId;
use husky_word::Ident;

// pub struct FluffyTypeMethodCard {
//     id: AssociatedItemId,
//     method_ty: FluffyCardResult<FluffyTerm>,
// }

pub(super) fn direct_ty_method_card(
    engine: &mut impl FluffyTermEngine,
    signature: TypeDeclarativeSignature,
    arguments: &[FluffyTerm],
    ident: Ident,
) -> FluffyCardResult<Option<FluffyMethodCard>> {
    todo!()
    // match self.data(engine) {
    //     FluffyTermData::Literal(_) => todo!(),
    //     FluffyTermData::TypeOntology { .. } => todo!(),
    //     FluffyTermData::PlaceTypeOntology {
    //         place,
    //         path,
    //         refined_path,
    //         arguments,
    //         base_ty_ethereal_term: base_ty_term,
    //     } => match base_ty_term {
    //         Some(base_ty_term) => {
    //             todo!()
    //             // let db = engine.db();
    //             // let Some(card) = base_ty_term.ty_method_card(db, ident)? else {
    //             //     return Ok(None)
    //             // };
    //             // Ok(Some(FluffyTypeMethodCard {
    //             //     id: card.id(db),
    //             //     method_ty: card.method_ty(db).map(Into::into).map_err(Into::into),
    //             // }))
    //         }
    //         None => todo!(),
    //     },
    //     FluffyTermData::Curry {
    //         curry_kind,
    //         variance,
    //         parameter_variable,
    //         parameter_ty,
    //         return_ty,
    //     } => todo!(),
    //     FluffyTermData::Hole(_, _) => todo!(),
    //     FluffyTermData::Category(_) => todo!(),
    //     FluffyTermData::Ritchie {
    //         ritchie_kind,
    //         parameter_contracted_tys,
    //         return_ty,
    //     } => todo!(),
    //     FluffyTermData::PlaceHole {
    //         place,
    //         hole_kind,
    //         hole,
    //     } => todo!(),
    // }
}
