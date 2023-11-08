use crate::*;
use husky_entity_path::FugitivePath;
use smallvec::SmallVec;

#[salsa::interned(db = ValDb, jar = ValJar)]
pub struct Val {
    pub opr: ValOpr,
    #[return_ref]
    pub opds: SmallVec<[Val; 2]>,
    pub domain: Option<ValDomain>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValDomain {
    ConditionSatisfied(Val),
    ConditionNotSatisfied(Val),
    StmtNotReturned(Val),
}

impl Val {
    pub unsafe fn from_raw(raw: u32) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum ValOpr {
    Fugitive(FugitivePath),
    Require,
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
