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
pub use branch::*;
pub use eval_id::*;
pub use expr::*;
use husky_pattern_semantics::ExprPattern;
use husky_vm_primitive_value::PrimitiveValueData;
use husky_xml_syntax::XmlTagKind;
pub use query::{FeatureGenQueryGroup, FeatureGenQueryGroupStorage, TrainModel};
pub use repr::*;
pub use stmt::{FeatureLazyStmtVariant, FeatureStmt};
pub use unique_allocate::{
    new_feature_interner, AllocateUniqueFeature, FeatureInterner, FeaturePtr,
};

use husky_compile_time::DeclQueryGroup;
use husky_entity_route::EntityRoutePtr;
use husky_entity_semantics::EntityDefnQueryGroup;
use husky_init_syntax::*;
use husky_loop_syntax::*;
use husky_opn_syntax::*;
use husky_print_utils::*;
use husky_text::*;
use husky_word::{CustomIdentifier, IdentPairDict};
use std::sync::Arc;
use temp::*;
use vm::EntityUid;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureSymbol {
    varname: CustomIdentifier,
    value: Arc<FeatureExpr>,
    feature: FeaturePtr,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Feature {
    Input, // ad hoc: needs to include task config
    PrimitiveLiteral(PrimitiveValueData),
    EnumLiteral(EntityRoutePtr),
    Assert {
        condition: FeaturePtr,
    },
    Require {
        condition: FeaturePtr,
    },
    Cascade(Vec<FeaturePtr>),
    PrimitiveBinaryOpr {
        opr: PureBinaryOpr,
        lopd: FeaturePtr,
        ropd: FeaturePtr,
    },
    CustomBinaryOpr {
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
    Index {
        opds: Vec<FeaturePtr>,
    },
    IndexFixed {
        this: FeaturePtr,
        index: usize,
    },
    CyclicIndexFixed {
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
    ArrivalAfterStmtNotReturn {
        stmt: FeaturePtr,
    },
    ArrivalAfterConditionNotMet {
        opt_parent: Option<FeaturePtr>,
        condition: FeaturePtr,
    },
    ArrivalIfConditionMet {
        opt_parent: Option<FeaturePtr>,
        condition: FeaturePtr,
    },
    NewVecFromList {
        elements: Vec<FeaturePtr>,
    },
    ExprPatternPrimitiveLiteral(FeaturePtr),
    ExprPatternOneOf {
        subpatterns: Vec<FeaturePtr>,
    },
    ExprPatternEnumLiteral(FeaturePtr),
    ExprPatternSome,
    ExprPatternNone,
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

    pub fn expr_pattern(patt: &ExprPattern) -> FeaturePtr {
        todo!()
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
