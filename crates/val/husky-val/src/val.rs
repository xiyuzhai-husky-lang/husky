use crate::*;
use husky_coword::Ident;
use husky_entity_path::{FugitivePath, TypeVariantPath};
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
    Keyed(Option<Val>),
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
    TypeVariant(TypeVariantPath),
    Be,
    Unwrap {},
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ValSuffixOpr;

impl ValSuffixOpr {
    pub fn from_hir(opr: HirSuffixOpr) -> Self {
        Self
    }
}
