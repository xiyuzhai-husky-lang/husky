mod block;
mod branch;
mod eval_id;
mod expr;
mod object;
mod query;
mod record;
mod repr;
mod stmt;
mod temp;
mod unique_allocate;
mod visual;

pub use block::*;
pub use branch::{FeatureBranch, FeatureBranchIndicator, FeatureBranchVariant};
pub use eval_id::*;
pub use expr::*;
pub use query::{FeatureGenQueryGroup, FeatureGenQueryGroupStorage};
pub use repr::*;
pub use stmt::{FeatureLazyStmtVariant, FeatureStmt};
pub use unique_allocate::{
    new_feature_interner, AllocateUniqueFeature, FeatureInterner, FeaturePtr,
};

use husky_compile_time::DeclQueryGroup;
use husky_entity_route::EntityRoutePtr;
use husky_entity_semantics::EntityDefnQueryGroup;
use husky_text::*;
use print_utils::*;
use std::sync::Arc;
use temp::*;
use vm::{CopyableValue, PureBinaryOpr};
use vm::{EntityUid, XmlTagKind};
use word::{CustomIdentifier, IdentPairDict};

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
    FunctionCall {
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
    CyclicElementAccessConstIndex {
        this: FeaturePtr,
        index: i32,
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
    Temp {
        uid: TempFeatureUid,
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
