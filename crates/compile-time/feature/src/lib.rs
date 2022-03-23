mod block;
mod branch;
mod eval;
mod expr;
mod query;
mod sheet;
mod stmt;
mod unique_allocate;

pub use block::FeatureBlock;
pub use branch::{FeatureBranch, FeatureBranchKind};
pub use eval::{eval_feature_block, eval_feature_expr, eval_feature_stmt, FeatureEvalIndicator};
pub use expr::{FeatureExpr, FeatureExprKind};
pub use query::{FeatureQueryGroup, FeatureQueryGroupStorage};
pub use sheet::FeatureSheet;
pub use stmt::{FeatureStmt, FeatureStmtKind};
pub use unique_allocate::{
    new_feature_unique_allocator, AllocateUniqueFeature, FeaturePtr, FeatureUniqueAllocator,
};

use print_utils::*;
use scope::ScopePtr;
use semantics_entity::EntityUid;
use std::sync::Arc;
use vm::{PrimitiveValue, PureBinaryOpr};
use word::CustomIdentifier;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureSymbol {
    varname: CustomIdentifier,
    value: Arc<FeatureExpr>,
    feature: FeaturePtr,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Feature {
    Input,
    PrimitiveLiteral(PrimitiveValue),
    EnumLiteral(ScopePtr),
    Assert {
        condition: FeaturePtr,
    },
    Block(Vec<FeaturePtr>),
    PrimitiveBinaryOpr {
        opr: PureBinaryOpr,
        lopd: FeaturePtr,
        ropd: FeaturePtr,
    },
    FuncCall {
        func: ScopePtr,
        uid: EntityUid,
        inputs: Vec<FeaturePtr>,
    },
    Branches {
        branches: Vec<BranchedFeature>,
    },
    MembVarAccess {
        this: FeaturePtr,
        memb_ident: CustomIdentifier,
    },
    MembCall {
        memb_ident: CustomIdentifier,
        opds: Vec<FeaturePtr>,
    },
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct BranchedFeature {
    condition: Option<FeaturePtr>,
    block: FeaturePtr,
}

impl From<&Feature> for Feature {
    fn from(feature: &Feature) -> Self {
        feature.clone()
    }
}
