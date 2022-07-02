use crate::*;
use husky_entity_route::{EntityRoutePtr, RangedEntityRoute};
use husky_text::RangedCustomIdentifier;
use infer_decl::TyDecl;
use std::sync::Arc;
use vm::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EagerOpnVariant {
    Binary {
        opr: BinaryOpr,
        this_ty: EntityRoutePtr,
    },
    Prefix {
        opr: PrefixOpr,
        this_ty: EntityRoutePtr,
    },
    Suffix {
        this_ty: EntityRoutePtr,
        opr: SuffixOpr,
    },
    RoutineCall(RangedEntityRoute),
    TypeCall {
        ranged_ty: RangedEntityRoute,
        ty_decl: Arc<TyDecl>,
    },
    FieldAccess {
        this_ty: EntityRoutePtr,
        field_ident: RangedCustomIdentifier,
        field_liason: MemberLiason,
        field_binding: Binding,
    },
    MethodCall {
        method_ident: RangedCustomIdentifier,
        this_ty_decl: Arc<TyDecl>,
        method_route: EntityRoutePtr,
        output_binding: Binding,
    },
    Index {
        element_binding: Binding,
    },
}
