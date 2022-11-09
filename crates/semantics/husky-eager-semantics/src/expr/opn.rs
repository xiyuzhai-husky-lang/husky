use crate::*;
use husky_entity_kind::FieldKind;
use husky_entity_route::{EntityRouteItd, RangedEntityRoute};
use husky_text::RangedCustomIdentifier;
use husky_vm::Binding;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EagerOpnVariant {
    Binary {
        opr: BinaryOpr,
    },
    Prefix {
        opr: PrefixOpr,
    },
    Suffix {
        opr: EagerSuffixOpr,
    },
    RoutineCall(RangedEntityRoute),
    ValueCall,
    TypeCall,
    NewVecFromList,
    Field {
        this_ty: EntityRouteItd,
        field_ident: RangedCustomIdentifier,
        field_liason: MemberModifier,
        field_binding: Binding,
        field_kind: FieldKind,
    },
    MethodCall {
        method_ident: RangedCustomIdentifier,
        method_route: EntityRouteItd,
        output_binding: Binding,
    },
    Index {
        element_binding: Binding,
    },
}
