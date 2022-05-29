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
    NormalRoutineCall(RangedEntityRoute),
    StructCall(RangedEntityRoute),
    RecordCall(RangedEntityRoute),
    PatternCall,
    FieldAccess {
        field_ident: RangedCustomIdentifier,
        field_kind: FieldKind,
        field_binding: Binding,
    },
    MethodCall {
        method_ident: RangedCustomIdentifier,
        method_route: EntityRoutePtr,
        opt_output_binding: Option<Binding>,
    },
    ElementAccess {
        element_binding: Binding,
    },
}
