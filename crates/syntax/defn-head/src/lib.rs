mod generic;

use entity_kind::RoutineKind;
use entity_route_query::EntityRouteQueryGroup;
pub use generic::*;
use static_defn::StaticInputPlaceholder;
use std::sync::Arc;

use entity_route::{EntityRoutePtr, RangedEntityRoute};
use vm::{FieldContract, InputContract, OutputContract};
use word::{CustomIdentifier, IdentDict};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RoutineDefnHead {
    pub ident: CustomIdentifier,
    pub routine_kind: RoutineKind,
    pub generic_placeholders: IdentDict<GenericPlaceholder>,
    pub input_placeholders: Arc<Vec<InputPlaceholder>>,
    pub output_ty: RangedEntityRoute,
    pub output_contract: OutputContract,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TypeMethodDefnHead {
    pub ident: CustomIdentifier,
    pub routine_kind: RoutineKind,
    pub this_contract: InputContract,
    pub generic_placeholders: IdentDict<GenericPlaceholder>,
    pub input_placeholders: Arc<Vec<InputPlaceholder>>,
    pub output_ty: RangedEntityRoute,
    pub output_contract: OutputContract,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FieldDefnHead {
    pub ident: CustomIdentifier,
    pub ty: EntityRoutePtr,
    pub kind: FieldKind,
    pub contract: FieldContract,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FieldKind {
    StructOriginal,
    StructDerived,
    RecordOriginal,
    RecordDerived,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputPlaceholder {
    pub ident: CustomIdentifier,
    pub contract: InputContract,
    pub ranged_ty: RangedEntityRoute,
}
