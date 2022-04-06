use std::sync::Arc;

use decl::{MembAccessKind, TyDecl};
use entity_route::{EntityRoutePtr, RangedEntityRoute};
use syntax_types::*;
use vm::PureBinaryOpr;
use word::CustomIdentifier;

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
        memb_ident: CustomIdentifier,
        memb_access_kind: MembAccessKind,
    },
    MembCall {
        memb_ident: CustomIdentifier,
    },
    ElementAccess,
}
