use super::*;
use husky_entity_path::path::{major_item::form::MajorFormPath, ty_variant::TypeVariantPath};
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_linket::linket::Linket;
use husky_term_prelude::literal::Literal;
use smallvec::SmallVec;

#[salsa::interned]
pub struct Genki {
    pub domain: GenkiDomain,
    pub opn: GenkiOpn,
    #[return_ref]
    pub arguments: SmallVec<[GenkiArgument; 4]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenkiArgument {
    Simple(Genki),
    Keyed(Option<Genki>),
    Variadic(Vec<Genki>),
    Branch {
        condition: Option<Genki>,
        stmts: SmallVec<[Genki; 4]>,
    },
    RuntimeConstants(SmallVec<[GenkiRuntimeConstant; 4]>),
}

#[salsa::interned]
pub struct GenkiRuntimeConstant {
    pub data: GenkiRuntimeConstantData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum GenkiRuntimeConstantData {
    TypeVariantPath(TypeVariantPath),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GenkiDomain {
    ConditionSatisfied(Genki),
    ConditionNotSatisfied(Genki),
    StmtNotTransferred(Genki),
    Omni,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum GenkiOpn {
    Var(/* todo: var id */),
    Return,
    Require,
    Assert,
    Val(MajorFormPath),
    FunctionRitchie(MajorFormPath),
    Prefix(HirPrefixOpr),
    Suffix(HirSuffixOpr),
    Binary(HirBinaryOpr),
    Linket(Linket),
    EvalDiscarded,
    Literal(Literal),
    Branches,
    TypeVariant(TypeVariantPath),
    /// use pattern_data instead of pattern for efficiency
    Be {
        pattern_data: KiPatternData,
    },
    Unwrap {},
    // ad hoc
    Index,
}
