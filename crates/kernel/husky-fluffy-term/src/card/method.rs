use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct FluffyMethodCard {
    place: Option<Place>,
    visibility: Visibility,
    modifier: FieldModifier,
    ty: FluffyTerm,
}
