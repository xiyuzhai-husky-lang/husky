use crate::*;
use husky_ast::FieldAstKind;
use husky_entity_kind::FieldKind;
use husky_entity_route::{EntityRoutePtr, RangedEntityRoute};
use husky_text::RangedCustomIdentifier;
use husky_vm::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LazyOpnKind {
    Binary {
        opr: PureBinaryOpr,
        this: EntityRoutePtr,
    },
    Prefix(PrefixOpr),
    FunctionModelCall(RangedEntityRoute),
    FunctionRoutineCall(RangedEntityRoute),
    StructCall(RangedEntityRoute),
    NewVecFromList,
    RecordCall(RangedEntityRoute),
    Field {
        field_ident: RangedCustomIdentifier,
        field_binding: Binding,
    },
    MethodCall {
        method_ident: RangedCustomIdentifier,
        method_route: EntityRoutePtr,
        output_binding: Binding,
    },
    Index {
        element_binding: Binding,
    },
}
