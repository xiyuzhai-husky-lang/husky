use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct FluffyMethodType {
    place: Option<Place>,
    visibility: Visibility,
    modifier: FieldModifier,
    ty: FluffyTerm,
}
