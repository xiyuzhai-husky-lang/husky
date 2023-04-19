mod trai_for_ty;
mod ty;

pub(crate) use self::trai_for_ty::*;
pub(crate) use self::ty::*;

use super::*;
use husky_word::Ident;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct FluffyMethodCard {
    indirections: SmallVec<[FluffyIndirection; 2]>,
    visibility: Visibility,
    ty: FluffyTerm,
}

impl FluffyTerm {
    pub(crate) fn method_card(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> FluffyCardResult<Option<FluffyMethodCard>> {
        self.method_card_aux(engine, ident, available_traits, smallvec![])
    }

    fn method_card_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
        mut indirections: SmallVec<[FluffyIndirection; 2]>,
    ) -> FluffyCardResult<Option<FluffyMethodCard>> {
        match self.data(engine) {
            FluffyTermData::TypeOntology {
                path,
                refined_path,
                arguments,
                ty_ethereal_term,
            } => method_card_aux(
                engine,
                path,
                refined_path,
                arguments.to_smallvec(),
                ty_ethereal_term,
                ident,
                available_traits,
                indirections,
            ),
            FluffyTermData::PlaceTypeOntology {
                place,
                path,
                refined_path,
                arguments,
                base_ty_ethereal_term,
            } => {
                indirections.push(FluffyIndirection::Place(place));
                method_card_aux(
                    engine,
                    path,
                    refined_path,
                    arguments.to_smallvec(),
                    base_ty_ethereal_term,
                    ident,
                    available_traits,
                    indirections,
                )
            }
            _ => Ok(None),
        }
    }
}

#[inline(always)]
fn method_card_aux(
    engine: &mut impl FluffyTermEngine,
    ty_path: TypePath,
    refined_path: Either<CustomTypePath, PreludeTypePath>,
    arguments: SmallVec<[FluffyTerm; 2]>,
    ty_ethereal_term: Option<EtherealTerm>,
    ident: Ident,
    available_traits: &[TraitPath],
    mut indirections: SmallVec<[FluffyIndirection; 2]>,
) -> FluffyCardResult<Option<FluffyMethodCard>> {
    match refined_path {
        Right(PreludeTypePath::Borrow(ty_path)) => match ty_path {
            PreludeBorrowTypePath::Ref => todo!(),
            PreludeBorrowTypePath::RefMut => todo!(),
            PreludeBorrowTypePath::Leash => {
                debug_assert_eq!(arguments.len(), 1);
                indirections.push(FluffyIndirection::Unleash);
                arguments[0].method_card_aux(engine, ident, available_traits, indirections)
            }
        },
        _ => {
            let signature = ty_path.declarative_signature_template(engine.db())?;
            if let Some(card) =
                direct_method_card(engine, ty_path, arguments, ident, available_traits)?
            {
                return Ok(Some(card));
            }
            // todo: consider `Deref` `DerefMut` `Carrier`
            Ok(None)
        }
    }
}

fn direct_method_card(
    engine: &mut impl FluffyTermEngine,
    ty_path: TypePath,
    arguments: SmallVec<[FluffyTerm; 2]>,
    ident: Ident,
    available_traits: &[TraitPath],
) -> FluffyCardResult<Option<FluffyMethodCard>> {
    if let Some(card) = direct_ty_method_card(engine, ty_path, &arguments, ident)? {
        return Ok(Some(card));
    }
    if let Some(card) =
        direct_trai_for_ty_method_card(engine, ty_path, &arguments, ident, available_traits)?
    {
        return Ok(Some(card));
    }
    Ok(None)
}
