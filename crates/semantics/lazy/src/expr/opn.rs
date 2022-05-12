use entity_kind::FieldKind;
use entity_route::{EntityRoutePtr, RangedEntityRoute};
use text::RangedCustomIdentifier;
use vm::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LazyOpnKind {
    Binary {
        opr: PureBinaryOpr,
        this: EntityRoutePtr,
    },
    Prefix(PrefixOpr),
    RoutineCall(RangedEntityRoute),
    StructCall(RangedEntityRoute),
    RecordCall(RangedEntityRoute),
    PatternCall,
    FieldAccess {
        field_ident: RangedCustomIdentifier,
        field_kind: FieldKind,
    },
    MethodCall {
        method_ident: RangedCustomIdentifier,
        method_route: EntityRoutePtr,
    },
    ElementAccess,
}
