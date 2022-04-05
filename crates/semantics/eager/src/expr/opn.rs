use std::sync::Arc;

use decl::TySignature;
use entity_route::{EntityRoutePtr, RangedScope};
use syntax_types::{PrefixOpr, SuffixOpr};
use vm::{BinaryOpr, MembAccessContract};
use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EagerOpnKind {
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
    RoutineCall(RangedScope),
    TypeCall {
        ranged_ty: RangedScope,
        ty_decl: Arc<TySignature>,
    },
    PatternCall,
    MembVarAccess {
        memb_var_contract: MembAccessContract,
    },
    MembRoutineCall {
        memb_ident: CustomIdentifier,
    },
    ElementAccess,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum PrefixOpn {}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum SuffixOpn {}
