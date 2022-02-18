mod block;
mod eval;
mod expr;
mod query;
mod sheet;
mod stmt;
mod unique_allocate;

use std::sync::Arc;

pub use block::FeatureBlock;
pub use eval::{eval_feature_block, eval_feature_expr, eval_feature_stmt};
pub use expr::{FeatureExpr, FeatureExprKind};
pub use query::{FeatureQueryGroup, FeatureQueryGroupStorage};
use scope::ScopePtr;
use semantics::EntityUid;
pub use sheet::FeatureSheet;
pub use stmt::{FeatureBranch, FeatureBranchKind, FeatureStmt, FeatureStmtKind};
pub use unique_allocate::{
    new_feature_unique_allocator, AllocateUniqueFeature, FeaturePtr, FeatureUniqueAllocator,
};

use common::*;
use vm::{BinaryOpr, PrimitiveValue};
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
    Literal(PrimitiveValue),
    Assert {
        condition: FeaturePtr,
    },
    Block(Vec<FeaturePtr>),
    PrimitiveBinaryOpr {
        opr: BinaryOpr,
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
