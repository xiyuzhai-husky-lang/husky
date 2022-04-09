mod generic;

use entity_syntax::RoutineKind;
pub use generic::*;
use std::sync::Arc;

use entity_route::{EntityRoutePtr, RangedEntityRoute};
use vm::{InputContract, MembAccessContract};
use word::{CustomIdentifier, IdentDict};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RoutineDefnHead {
    pub ident: CustomIdentifier,
    pub routine_kind: RoutineKind,
    pub generic_placeholders: IdentDict<GenericPlaceholder>,
    pub input_placeholders: Arc<Vec<InputPlaceholder>>,
    pub output: RangedEntityRoute,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MembRoutineDefnHead {
    pub ident: CustomIdentifier,
    pub routine_kind: RoutineKind,
    pub this_contract: InputContract,
    pub generics: IdentDict<GenericPlaceholder>,
    pub input_placeholders: Arc<Vec<InputPlaceholder>>,
    pub output: RangedEntityRoute,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FieldDefnHead {
    pub ident: CustomIdentifier,
    pub contract: MembAccessContract,
    pub ty: EntityRoutePtr,
    pub kind: FieldKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FieldKind {
    Original,
    Derived,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputPlaceholder {
    pub ident: CustomIdentifier,
    pub contract: InputContract,
    pub ranged_ty: RangedEntityRoute,
}
