use crate::*;
use husky_entity_path::path::{major_item::form::MajorFormPath, ty_variant::TypeVariantPath};
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_linket::linket::Linket;
use husky_term_prelude::literal::Literal;
use smallvec::SmallVec;

#[salsa::interned]
pub struct Ki {
    pub domain: KiDomain,
    pub opn: KiOpn,
    #[return_ref]
    pub arguments: SmallVec<[KiArgument; 4]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KiArgument {
    Simple(Ki),
    Keyed(Option<Ki>),
    Variadic(Vec<Ki>),
    Branch {
        condition: Option<Ki>,
        stmts: SmallVec<[Ki; 4]>,
    },
    RuntimeCompterms(SmallVec<[KiRuntimeCompterm; 4]>),
}

#[salsa::interned]
pub struct KiRuntimeCompterm {
    pub data: KiRuntimeComptermData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum KiRuntimeComptermData {
    TypeVariantPath(TypeVariantPath),
}

#[test]
fn ki_runtime_compterms_works() {
    use husky_ki_repr_interface::KiRuntimeComptermInterface;

    assert_eq!(
        std::mem::size_of::<KiRuntimeCompterm>(),
        std::mem::size_of::<KiRuntimeComptermInterface>(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KiDomain {
    ConditionSatisfied(Ki),
    ConditionNotSatisfied(Ki),
    ControlNotTransferred(Ki),
    Omni,
}

impl Ki {
    pub unsafe fn from_raw(_raw: u32) -> Self {
        todo!()
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum KiOpn {
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

#[salsa::interned]
pub struct KiPattern {
    data: KiPatternData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum KiPatternData {
    None,
    Some,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct KiSuffixOpr;

impl KiSuffixOpr {
    pub fn from_hir(_opr: HirSuffixOpr) -> Self {
        Self
    }
}
