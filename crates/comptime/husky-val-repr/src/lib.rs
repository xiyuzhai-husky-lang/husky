pub mod db;
mod domain;
mod expr;
mod repr;
mod stmt;

pub use self::domain::*;
pub use self::expr::*;
pub use self::repr::*;
pub use self::stmt::*;

use self::db::*;
use husky_entity_syn_tree::RegionPath;

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
// use husky_entity_path::EntityPath;
// use husky_opn_semantics::ImplicitConversion;
// use husky_pattern_semantics::{PurePattern, PurePatternVariant};
// use husky_vm_primitive_value::PrimitiveValueData;
// use husky_xml_syntax::HtmlTagKind;
// pub use intern::{FeatureInterner, Val, InternFeature};
// pub use lazy_branch::*;
// pub use lazy_expr::*;
// pub use lazy_stmt::{ValStmt, ValStmtData};
// pub use query::{ValReprDb, FeatureGenQueryGroupStorage, TrainModel};
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
// pub struct ValSymbol {
//     varname: Ident,
//     value: ValExpr,
//     feature: Val,
// }

// #[derive(Debug, Hash, PartialEq, Eq, Clone)]
// pub enum Feature {
//     Input, // ad hoc: needs to include task config
//     PrimitiveLiteral(PrimitiveValueData),
//     EnumLiteral(EtherealTerm),
//     Assert {
//         condition: Val,
//     },
//     Require {
//         condition: Val,
//     },
//     ReturnUnveil {
//         result: Val,
//         implicit_conversion: ImplicitConversion,
//     },
//     Cascade(Vec<Val>),
//     PrimitiveBinaryOpr {
//         opr: BinaryClosedOpr,
//         lopd: Val,
//         ropd: Val,
//     },
//     CustomBinaryOpr {
//         opr: BinaryClosedOpr,
//         lopd: Val,
//         ropd: Val,
//     },
//     FunctionCall {
//         func: EtherealTerm,
//         uid: EntityUid,
//         inputs: Vec<Val>,
//     },
//     Branches {
//         branches: Vec<BranchedFeature>,
//     },
//     FieldAccess {
//         this: Val,
//         field_ident: Ident,
//     },
//     Index {
//         opds: Vec<Val>,
//     },
//     IndexFixed {
//         this: Val,
//         index: usize,
//     },
//     CyclicIndexFixed {
//         this: Val,
//         index: i32,
//     },
//     MethodCall {
//         method_ident: Ident,
//         opds: Vec<Val>,
//     },
//     EntityFeature {
//         item_path: EntityPath,
//         uid: EntityUid,
//     },
//     RecordTypeCall {
//         ty: HirType,
//         uid: EntityUid,
//         opds: Vec<Val>,
//     },
//     HtmlFromValue {
//         value: Val,
//     },
//     HtmlFromTag {
//         tag_kind: HtmlTagKind,
//         props: IdentPairMap<Val>,
//     },
//     Temp {
//         uid: TempFeatureUid,
//     },
//     ArrivalAfterStmtNotReturn {
//         stmt: Val,
//         // without opt_stmt_arrival_indicator, there will be clash
//         opt_stmt_arrival_indicator: Option<Val>,
//     },
//     ArrivalAfterConditionNotMet {
//         opt_parent: Option<Val>,
//         condition: Val,
//     },
//     ArrivalIfConditionMet {
//         opt_parent: Option<Val>,
//         condition: Val,
//     },
//     NewVecFromList {
//         elements: Vec<Val>,
//     },
//     PurePatternPrimitiveLiteral(Val),
//     PurePatternOneOf {
//         subpatterns: Vec<Val>,
//     },
//     PurePatternEnumLiteral(Val),
//     PurePatternSome,
//     PurePatternNone,
//     BePattern {
//         this: Val,
//         expr_pattern: Val,
//     },
// }

// impl Feature {
//     pub fn intern_block(interner: &FeatureInterner, stmts: &[ValStmt]) -> Val {
//         let stmt_features: Vec<_> = stmts.iter().filter_map(|stmt| stmt.opt_feature).collect();
//         if stmt_features.len() == 1 {
//             stmt_features[0]
//         } else {
//             interner.intern(Feature::Cascade(stmt_features))
//         }
//     }

//     pub fn intern_expr_pattern(interner: &FeatureInterner, patt: &PurePattern) -> Val {
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
//     condition: Option<Val>,
//     block: Val,
// }

// impl From<&Feature> for Feature {
//     fn from(feature: &Feature) -> Self {
//         feature.clone()
//     }
// }
