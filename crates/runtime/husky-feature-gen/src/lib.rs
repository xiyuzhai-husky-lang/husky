mod block;
mod eval_id;
mod intern;
mod lazy_branch;
mod lazy_expr;
mod lazy_stmt;
mod object;
mod query;
mod record;
mod repr;
mod temp;
mod visual;

pub use block::*;
pub use eval_id::*;
use husky_opn_semantics::ImplicitConversion;
use husky_pattern_semantics::{PurePattern, PurePatternVariant};
use husky_vm_primitive_value::PrimitiveValueData;
use husky_xml_syntax::XmlTagKind;
pub use intern::{FeatureInterner, FeaturePtr, InternFeature};
pub use lazy_branch::*;
pub use lazy_expr::*;
pub use lazy_stmt::{FeatureLazyStmt, FeatureLazyStmtVariant};
pub use query::{FeatureGenQueryGroup, FeatureGenQueryGroupStorage, TrainModel};
pub use repr::*;

use husky_comptime::DeclQueryGroup;
use husky_entity_route::EntityRoutePtr;
use husky_entity_semantics::EntityDefnQueryGroup;
use husky_opn_syntax::*;
use husky_print_utils::*;
use husky_text::*;
use husky_vm::EntityUid;
use husky_word::{CustomIdentifier, IdentPairDict};
use std::sync::Arc;
use temp::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureSymbol {
    varname: CustomIdentifier,
    value: Arc<FeatureLazyExpr>,
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
    ReturnUnveil {
        result: FeaturePtr,
        implicit_conversion: ImplicitConversion,
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
        // without opt_stmt_arrival_indicator, there will be clash
        opt_stmt_arrival_indicator: Option<FeaturePtr>,
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
    PurePatternPrimitiveLiteral(FeaturePtr),
    PurePatternOneOf {
        subpatterns: Vec<FeaturePtr>,
    },
    PurePatternEnumLiteral(FeaturePtr),
    PurePatternSome,
    PurePatternNone,
    BePattern {
        this: FeaturePtr,
        expr_pattern: FeaturePtr,
    },
}

impl Feature {
    pub fn intern_block(interner: &FeatureInterner, stmts: &[Arc<FeatureLazyStmt>]) -> FeaturePtr {
        let stmt_features: Vec<_> = stmts.iter().filter_map(|stmt| stmt.opt_feature).collect();
        if stmt_features.len() == 1 {
            stmt_features[0]
        } else {
            interner.intern(Feature::Cascade(stmt_features))
        }
    }

    pub fn intern_expr_pattern(interner: &FeatureInterner, patt: &PurePattern) -> FeaturePtr {
        match patt.variant {
            PurePatternVariant::PrimitiveLiteral(_) => todo!(),
            PurePatternVariant::OneOf { .. } => todo!(),
            PurePatternVariant::EnumLiteral(_) => todo!(),
            PurePatternVariant::Some => interner.intern(Feature::PurePatternSome),
            PurePatternVariant::None => interner.intern(Feature::PurePatternNone),
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
