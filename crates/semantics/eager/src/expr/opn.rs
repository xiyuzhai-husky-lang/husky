use std::sync::Arc;

use entity_route::{EntityRoutePtr, RangedEntityRoute};
use infer_decl::TyDecl;
use syntax_types::{PrefixOpr, SuffixOpr};
use vm::{BinaryOpr, FieldContract};
use word::RangedCustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EagerOpnVariant {
    Binary {
        opr: BinaryOpr,
        this: EntityRoutePtr,
    },
    Prefix {
        opr: PrefixOpr,
        this: EntityRoutePtr,
    },
    Suffix {
        opr: SuffixOpr,
        this: EntityRoutePtr,
    },
    RoutineCall(RangedEntityRoute),
    TypeCall {
        ranged_ty: RangedEntityRoute,
        ty_decl: Arc<TyDecl>,
    },
    PatternCall,
    FieldAccess {
        field_contract: FieldContract,
    },
    MethodCall {
        method_ident: RangedCustomIdentifier,
        this_ty_decl: Arc<TyDecl>,
        method_route: EntityRoutePtr,
    },
    ElementAccess,
}
