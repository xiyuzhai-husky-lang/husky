mod generic;

use entity_kind::FieldKind;
pub use generic::*;
use std::sync::Arc;

use entity_route::{EntityRoutePtr, RangedEntityRoute};
use text::RangedCustomIdentifier;
use vm::{FieldContract, InputContract, OutputLiason};
use word::{CustomIdentifier, IdentDict, RoutineKeyword};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RoutineDefnHead {
    pub ident: RangedCustomIdentifier,
    pub routine_kind: RoutineKeyword,
    pub generic_placeholders: IdentDict<GenericPlaceholder>,
    pub parameters: Arc<Vec<InputParameter>>,
    pub output_ty: RangedEntityRoute,
    pub output_liason: OutputLiason,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TypeMethodDefnHead {
    pub ident: RangedCustomIdentifier,
    pub routine_kind: RoutineKeyword,
    pub this_contract: InputContract,
    pub generic_placeholders: IdentDict<GenericPlaceholder>,
    pub input_placeholders: Arc<Vec<InputParameter>>,
    pub output_ty: RangedEntityRoute,
    pub output_contract: OutputLiason,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FieldDefnHead {
    pub ident: RangedCustomIdentifier,
    pub ty: EntityRoutePtr,
    pub kind: FieldKind,
    pub contract: FieldContract,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputParameter {
    pub ident: RangedCustomIdentifier,
    pub contract: InputContract,
    pub ranged_ty: RangedEntityRoute,
}
