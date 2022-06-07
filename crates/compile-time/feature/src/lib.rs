mod block;
mod branch;
mod eval_id;
mod expr;
mod object;
mod query;
mod record;
mod repr;
mod stmt;
mod unique_allocate;

pub use block::*;
pub use branch::{FeatureBranch, FeatureBranchVariant};
pub use eval_id::*;
pub use expr::{FeatureExpr, FeatureExprVariant};
pub use query::{FeatureGenQueryGroup, FeatureGenQueryGroupStorage};
pub use repr::*;
pub use stmt::{FeatureStmt, FeatureStmtVariant};
pub use unique_allocate::{
    new_feature_unique_allocator, AllocateUniqueFeature, FeaturePtr, FeatureUniqueAllocator,
};

use entity_route::EntityRoutePtr;
use print_utils::*;
use std::sync::Arc;
use text::*;
use vm::EntityUid;
use vm::{CopyableValue, PureBinaryOpr};
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
    PrimitiveLiteral(CopyableValue),
    EnumLiteral(EntityRoutePtr),
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
        func: EntityRoutePtr,
        uid: EntityUid,
        inputs: Vec<FeaturePtr>,
    },
    Branches {
        branches: Vec<BranchedFeature>,
    },
    FieldAccess {
        this: FeaturePtr,
        field_ident: CustomIdentifier,
    },
    ElementAccess {
        opds: Vec<FeaturePtr>,
    },
    MethodCall {
        method_ident: CustomIdentifier,
        opds: Vec<FeaturePtr>,
    },
    EntityFeature {
        route: EntityRoutePtr,
        uid: EntityUid,
    },
    RecordTypeCall {
        ty: EntityRoutePtr,
        uid: EntityUid,
        opds: Vec<FeaturePtr>,
    },
}

impl Feature {
    pub fn block(features: &FeatureUniqueAllocator, stmts: &[Arc<FeatureStmt>]) -> FeaturePtr {
        let stmt_features: Vec<_> = stmts.iter().filter_map(|stmt| stmt.opt_feature).collect();
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
