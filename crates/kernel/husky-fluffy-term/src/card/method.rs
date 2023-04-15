mod trai_for_ty;
mod ty;

pub(crate) use self::trai_for_ty::*;
pub(crate) use self::ty::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct FluffyMethodCard {
    place: Option<Place>,
    visibility: Visibility,
    modifier: FieldModifier,
    ty: FluffyTerm,
}
