use std::sync::Arc;

use infer_signature::TySignature;
use scope::{RangedScope, ScopePtr};
use syntax_types::{PrefixOpr, SuffixOpr};
use vm::{BinaryOpr, MembAccessContract};
use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EagerOpnKind {
    Binary {
        opr: BinaryOpr,
        this: ScopePtr,
    },
    Prefix {
        opr: PrefixOpr,
        this: ScopePtr,
    },
    Suffix {
        opr: SuffixOpr,
        this: ScopePtr,
    },
    RoutineCall(RangedScope),
    TypeCall {
        ranged_ty: RangedScope,
        ty_signature: Arc<TySignature>,
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
