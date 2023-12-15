use crate::*;
use husky_coword::Ident;
use husky_entity_path::FugitivePath;
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_linkage::linkage::Linkage;
use husky_term_prelude::TermLiteral;
use smallvec::SmallVec;

#[salsa::interned(db = ValDb, jar = ValJar)]
pub struct Val {
    pub domain: ValDomain,
    pub opn: ValOpn,
    #[return_ref]
    pub arguments: SmallVec<[ValArgument; 4]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValArgument {
    Ordinary(Val),
    Keyed(Ident, Option<Val>),
    Variadic(Vec<Val>),
    Branch {
        condition: Option<Val>,
        stmts: SmallVec<[Val; 4]>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValDomain {
    ConditionSatisfied(Val),
    ConditionNotSatisfied(Val),
    StmtNotReturned(Val),
    Omni,
}

impl Val {
    pub unsafe fn from_raw(raw: u32) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = ValDb, jar = ValJar)]
pub enum ValOpn {
    Return,
    Require,
    Assert,
    ValItemLazilyDefined(FugitivePath),
    FunctionGn(FugitivePath),
    Prefix(HirPrefixOpr),
    Suffix(HirSuffixOpr),
    Binary(HirBinaryOpr),
    Linkage(Linkage),
    EvalDiscarded,
    Literal(TermLiteral),
    NewList,
    Branches,
    TypeVariant(husky_entity_path::TypeVariantPath),
    Be,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ValSuffixOpr;

impl ValSuffixOpr {
    pub fn from_hir(opr: HirSuffixOpr) -> Self {
        Self
    }
}

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
