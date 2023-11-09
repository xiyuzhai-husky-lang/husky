use crate::*;
use husky_entity_path::FugitivePath;
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_linkage_path::LinkagePath;
use smallvec::SmallVec;

#[salsa::interned(db = ValDb, jar = ValJar, override_debug)]
pub struct Val {
    pub domain: ValDomain,
    pub opn: ValOpn,
    #[return_ref]
    pub opds: SmallVec<[Val; 2]>,
}

impl<_Db: ValDb + ?Sized> ::salsa::DebugWithDb<_Db> for Val {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        _db: &_Db,
        _level: salsa::DebugFormatLevel,
    ) -> ::std::fmt::Result {
        #[allow(unused_imports)]
        use ::salsa::debug::helper::Fallback;
        let _db = <_Db as ::salsa::DbWithJar<ValJar>>::as_jar_db(_db);
        let mut debug_struct = &mut f.debug_struct("Val");
        if _level.is_root() {
            debug_struct = debug_struct.field("[salsa id]", &self.0.as_u32());
        }
        debug_struct =
            debug_struct.field(
                "domain",
                &::salsa::debug::helper::SalsaDebug::<
                    ValDomain,
                    <ValJar as salsa::jar::Jar<'_>>::DynDb,
                >::salsa_debug(
                    #[allow(clippy::needless_borrow)]
                    &self.domain(_db),
                    _db,
                    _level.next(),
                ),
            );
        debug_struct = debug_struct.field("opn", & ::salsa::debug::helper::SalsaDebug:: <ValOpn, <ValJar as salsa::jar::Jar<'_> > ::DynDb> ::salsa_debug(#[allow(clippy::needless_borrow)]
    &self.opn(_db),_db,_level.next()));
        debug_struct = debug_struct.field("opds", &self.opds(_db));
        debug_struct.finish()
    }
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
pub enum ValOpn {
    ValItem(FugitivePath),
    FunctionGn(FugitivePath),
    Require,
    Prefix(HirPrefixOpr),
    Suffix(HirSuffixOpr),
    Binary(HirBinaryOpr),
    Linkage(LinkagePath),
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
