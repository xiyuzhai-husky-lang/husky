mod block;
mod branch;
mod eval;
mod expr;
mod query;
mod sheet;
mod stmt;
mod unique_allocate;

use std::sync::Arc;

pub use block::LazyBlock;
pub use branch::{LazyBranch, LazyBranchKind};
pub use eval::{eval_lazy_block, eval_lazy_expr, eval_lazy_stmt, FeatureEvalIndicator};
pub use expr::{LazyExpr, LazyExprKind};
pub use query::{FeatureQueryGroup, FeatureQueryGroupStorage};
use scope::ScopePtr;
use semantics::EntityUid;
pub use sheet::FeatureSheet;
pub use stmt::{LazyStmt, LazyStmtKind};
pub use unique_allocate::{
    new_feature_unique_allocator, AllocateUniqueFeature, FeaturePtr, FeatureUniqueAllocator,
};

use common::*;
use vm::{PrimitiveValue, PureBinaryOpr};
use word::CustomIdentifier;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LazySymbol {
    varname: CustomIdentifier,
    value: Arc<LazyExpr>,
    feature: FeaturePtr,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Feature {
    Input,
    Literal(PrimitiveValue),
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
