mod regular;
mod ty_memo;

use self::regular::*;
use self::ty_memo::*;
use super::*;
use husky_word::Ident;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct FluffyFieldCard {
    indirections: SmallVec<[FluffyIndirection; 2]>,
    owner_ty_path: TypePath,
    visibility: Visibility,
    modifier: FieldModifier,
    ty: FluffyTerm,
}

impl FluffyFieldCard {
    fn to_term(self, engine: &mut impl FluffyTermEngine) -> FluffyTerm {
        todo!()
    }
}

impl FluffyTerm {
    pub(crate) fn field_card(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> FluffyCardResult<Option<FluffyFieldCard>> {
        self.field_card_aux(engine, ident, available_traits, smallvec![])
    }

    fn field_card_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
        mut indirections: SmallVec<[FluffyIndirection; 2]>,
    ) -> FluffyCardResult<Option<FluffyFieldCard>> {
        match self.data(engine) {
            FluffyTermData::TypeOntology {
                path,
                refined_path,
                arguments,
                ty_ethereal_term,
            } => field_card_aux(
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
                field_card_aux(
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
fn field_card_aux(
    engine: &mut impl FluffyTermEngine,
    ty_path: TypePath,
    refined_path: Either<CustomTypePath, PreludeTypePath>,
    arguments: SmallVec<[FluffyTerm; 2]>,
    ty_ethereal_term: Option<EtherealTerm>,
    ident: Ident,
    available_traits: &[TraitPath],
    mut indirections: SmallVec<[FluffyIndirection; 2]>,
) -> FluffyCardResult<Option<FluffyFieldCard>> {
    match refined_path {
        Right(PreludeTypePath::Borrow(ty_path)) => match ty_path {
            PreludeBorrowTypePath::Ref => todo!(),
            PreludeBorrowTypePath::RefMut => todo!(),
            PreludeBorrowTypePath::Leash => {
                debug_assert_eq!(arguments.len(), 1);
                indirections.push(FluffyIndirection::Unleash);
                arguments[0].field_card_aux(engine, ident, available_traits, indirections)
            }
        },
        _ => {
            let signature = ty_path.signature(engine.db())?;
            if let Some(card) = direct_field_card(engine, signature, arguments, ident)? {
                return Ok(Some(card));
            }
            // todo: consider `Deref` `DerefMut` `Carrier`
            Ok(None)
        }
    }
}

fn direct_field_card(
    engine: &mut impl FluffyTermEngine,
    signature: TypeSignature,
    arguments: SmallVec<[FluffyTerm; 2]>,
    ident: Ident,
) -> FluffyCardResult<Option<FluffyFieldCard>> {
    if let Some(card) = direct_regular_field_card(engine, signature, &arguments, ident)? {
        return Ok(Some(card));
    }
    if let Some(card) = direct_ty_memo_field_card(engine, signature, &arguments, ident)? {
        return Ok(Some(card));
    }
    Ok(None)
}
