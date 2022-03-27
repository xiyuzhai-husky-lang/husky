mod branch;
mod eval;
mod expr;
mod object;
mod query;
mod record;
mod repr;
mod sheet;
mod stmt;
mod this;
mod unique_allocate;

pub use branch::{FeatureBranch, FeatureBranchKind};
pub use eval::{eval_feature_block, eval_feature_expr, eval_feature_stmt, FeatureEvalIndicator};
pub use expr::{FeatureExpr, FeatureExprKind};
pub use query::{FeatureQueryGroup, FeatureQueryGroupStorage};
pub use repr::*;
pub use sheet::FeatureSheet;
pub use stmt::{FeatureStmt, FeatureStmtKind};
pub use this::FeatureBlock;
pub use unique_allocate::{
    new_feature_unique_allocator, AllocateUniqueFeature, FeaturePtr, FeatureUniqueAllocator,
};

use eval::*;
use object::Object;
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
    Cascade(Vec<FeaturePtr>),
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
    StructMembVarAccess {
        this: FeaturePtr,
        memb_ident: CustomIdentifier,
    },
    MembCall {
        memb_ident: CustomIdentifier,
        opds: Vec<FeaturePtr>,
    },
    ScopedFeature {
        scope: ScopePtr,
        uid: EntityUid,
    },
    ClassCall {
        ty: ScopePtr,
        uid: EntityUid,
        opds: Vec<FeaturePtr>,
    },
}

impl Feature {
    pub fn block(features: &FeatureUniqueAllocator, stmts: &[Arc<FeatureStmt>]) -> FeaturePtr {
        let stmt_features: Vec<_> = stmts.iter().filter_map(|stmt| stmt.feature).collect();
        if stmt_features.len() == 1 {
            stmt_features[0]
        } else {
            features.alloc(Feature::Cascade(stmt_features))
        }
    }
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
