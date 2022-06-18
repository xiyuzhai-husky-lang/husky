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
mod visual;

pub use block::*;
pub use branch::{FeatureBranch, FeatureBranchVariant};
pub use eval_id::*;
pub use expr::*;
pub use query::{FeatureGenQueryGroup, FeatureGenQueryGroupStorage};
pub use repr::*;
pub use stmt::{FeatureStmt, FeatureStmtVariant};
pub use unique_allocate::{
    new_feature_unique_allocator, AllocateUniqueFeature, FeatureInterner, FeaturePtr,
};

use entity_route::EntityRoutePtr;
use print_utils::*;
use std::sync::Arc;
use text::*;
use vm::{CopyableValue, PureBinaryOpr};
use vm::{EntityUid, XmlTagKind};
use word::{CustomIdentifier, IdentPairDict};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureSymbol {
    varname: CustomIdentifier,
    value: Arc<FeatureLazyExpr>,
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
    ElementAccessConstIndex {
        this: FeaturePtr,
        index: usize,
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
    XmlFromValue {
        value: FeaturePtr,
    },
    XmlFromTag {
        tag_kind: XmlTagKind,
        props: IdentPairDict<FeaturePtr>,
    },
}

impl Feature {
    pub fn block(features: &FeatureInterner, stmts: &[Arc<FeatureStmt>]) -> FeaturePtr {
        let stmt_features: Vec<_> = stmts.iter().filter_map(|stmt| stmt.opt_feature).collect();
        if stmt_features.len() == 1 {
            stmt_features[0]
        } else {
            features.intern(Feature::Cascade(stmt_features))
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
