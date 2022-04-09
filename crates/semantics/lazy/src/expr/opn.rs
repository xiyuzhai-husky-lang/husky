use std::sync::Arc;

use entity_route::{EntityRoutePtr, RangedEntityRoute};
use infer_decl::{FieldAccessKind, TyDecl};
use syntax_types::*;
use vm::PureBinaryOpr;
use word::{CustomIdentifier, RangedCustomIdentifier};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LazyOpnKind {
    Binary {
        opr: PureBinaryOpr,
        this: EntityRoutePtr,
    },
    Prefix(PrefixOpr),
    RoutineCall(RangedEntityRoute),
    StructCall(RangedEntityRoute),
    ClassCall(RangedEntityRoute),
    PatternCall,
    MembAccess {
        field_ident: RangedCustomIdentifier,
        field_access_kind: FieldAccessKind,
    },
    MembCall {
        field_ident: RangedCustomIdentifier,
    },
    ElementAccess,
}
