// mod block;
// mod eval_id;
// mod intern;
// mod lazy_branch;
// mod lazy_expr;
// mod lazy_stmt;
// mod object;
// mod query;
// mod record;
// mod repr;
// mod temp;
// mod visual;

// pub use block::*;
// pub use eval_id::*;
// use husky_item_path::EntityPath;
// use husky_opn_semantics::ImplicitConversion;
// use husky_pattern_semantics::{PurePattern, PurePatternVariant};
// use husky_vm_primitive_value::PrimitiveValueData;
// use husky_xml_syntax::HtmlTagKind;
// pub use intern::{FeatureInterner, FeatureItd, InternFeature};
// pub use lazy_branch::*;
// pub use lazy_expr::*;
// pub use lazy_stmt::{FeatureLazyStmt, FeatureLazyStmtVariant};
// pub use query::{FeatureGenQueryGroup, FeatureGenQueryGroupStorage, TrainModel};
// pub use repr::*;

// use husky_item_semantics::EntityDefnQueryGroup;
// use husky_coword::{IdentPairMap, Ident};
// use husky_opr::*;
// use husky_print_utils::*;
// use husky_ethereal_term::EtherealTerm;
// use husky_text::*;
// use husky_vm::EntityUid;
// use std::sync::Arc;
// use temp::*;

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub struct FeatureSymbol {
//     varname: Ident,
//     value: Arc<FeatureLazyExpr>,
//     feature: FeatureItd,
// }

// #[derive(Debug, Hash, PartialEq, Eq, Clone)]
// pub enum Feature {
//     Input, // ad hoc: needs to include task config
//     PrimitiveLiteral(PrimitiveValueData),
//     EnumLiteral(EtherealTerm),
//     Assert {
//         condition: FeatureItd,
//     },
//     Require {
//         condition: FeatureItd,
//     },
//     ReturnUnveil {
//         result: FeatureItd,
//         implicit_conversion: ImplicitConversion,
//     },
//     Cascade(Vec<FeatureItd>),
//     PrimitiveBinaryOpr {
//         opr: BinaryPureClosedOpr,
//         lopd: FeatureItd,
//         ropd: FeatureItd,
//     },
//     CustomBinaryOpr {
//         opr: BinaryPureClosedOpr,
//         lopd: FeatureItd,
//         ropd: FeatureItd,
//     },
//     FunctionCall {
//         func: EtherealTerm,
//         uid: EntityUid,
//         inputs: Vec<FeatureItd>,
//     },
//     Branches {
//         branches: Vec<BranchedFeature>,
//     },
//     FieldAccess {
//         this: FeatureItd,
//         field_ident: Ident,
//     },
//     Index {
//         opds: Vec<FeatureItd>,
//     },
//     IndexFixed {
//         this: FeatureItd,
//         index: usize,
//     },
//     CyclicIndexFixed {
//         this: FeatureItd,
//         index: i32,
//     },
//     MethodCall {
//         method_ident: Ident,
//         opds: Vec<FeatureItd>,
//     },
//     EntityFeature {
//         item_path: EntityPath,
//         uid: EntityUid,
//     },
//     RecordTypeCall {
//         ty: EtherealTerm,
//         uid: EntityUid,
//         opds: Vec<FeatureItd>,
//     },
//     HtmlFromValue {
//         value: FeatureItd,
//     },
//     HtmlFromTag {
//         tag_kind: HtmlTagKind,
//         props: IdentPairMap<FeatureItd>,
//     },
//     Temp {
//         uid: TempFeatureUid,
//     },
//     ArrivalAfterStmtNotReturn {
//         stmt: FeatureItd,
//         // without opt_stmt_arrival_indicator, there will be clash
//         opt_stmt_arrival_indicator: Option<FeatureItd>,
//     },
//     ArrivalAfterConditionNotMet {
//         opt_parent: Option<FeatureItd>,
//         condition: FeatureItd,
//     },
//     ArrivalIfConditionMet {
//         opt_parent: Option<FeatureItd>,
//         condition: FeatureItd,
//     },
//     NewVecFromList {
//         elements: Vec<FeatureItd>,
//     },
//     PurePatternPrimitiveLiteral(FeatureItd),
//     PurePatternOneOf {
//         subpatterns: Vec<FeatureItd>,
//     },
//     PurePatternEnumLiteral(FeatureItd),
//     PurePatternSome,
//     PurePatternNone,
//     BePattern {
//         this: FeatureItd,
//         expr_pattern: FeatureItd,
//     },
// }

// impl Feature {
//     pub fn intern_block(interner: &FeatureInterner, stmts: &[Arc<FeatureLazyStmt>]) -> FeatureItd {
//         let stmt_features: Vec<_> = stmts.iter().filter_map(|stmt| stmt.opt_feature).collect();
//         if stmt_features.len() == 1 {
//             stmt_features[0]
//         } else {
//             interner.intern(Feature::Cascade(stmt_features))
//         }
//     }

//     pub fn intern_expr_pattern(interner: &FeatureInterner, patt: &PurePattern) -> FeatureItd {
//         match patt.variant {
//             PurePatternVariant::PrimitiveLiteral(_) => todo!(),
//             PurePatternVariant::OneOf { .. } => todo!(),
//             PurePatternVariant::EnumLiteral(_) => todo!(),
//             PurePatternVariant::Some => interner.intern(Feature::PurePatternSome),
//             PurePatternVariant::None => interner.intern(Feature::PurePatternNone),
//         }
//     }
// }

// #[derive(Debug, Hash, PartialEq, Eq, Clone)]
// pub struct BranchedFeature {
//     condition: Option<FeatureItd>,
//     block: FeatureItd,
// }

// impl From<&Feature> for Feature {
//     fn from(feature: &Feature) -> Self {
//         feature.clone()
//     }
// }
mod object;
