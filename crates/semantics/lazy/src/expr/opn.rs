use defn_head::FieldKind;
use entity_route::{EntityRoutePtr, RangedEntityRoute};
use syntax_types::*;
use vm::PureBinaryOpr;
use word::RangedCustomIdentifier;

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
        field_kind: FieldKind,
    },
    MethodCall {
        method_ident: RangedCustomIdentifier,
        method_route: EntityRoutePtr,
    },
    ElementAccess,
}
