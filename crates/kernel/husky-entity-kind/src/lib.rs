pub mod ritchie;

use self::ritchie::*;
#[cfg(feature = "protocol_support")]
use husky_entity_protocol::*;
use serde::{Deserialize, Serialize};

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeKind {
    Enum,
    Inductive,
    Record,
    Struct,
    Structure,
    Extern,
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MajorFugitiveKind {
    Ritchie(RitchieItemKind),
    TypeAlias,
    Val,
    Formal,
    Const,
}

impl MajorFugitiveKind {
    pub const FN: Self = MajorFugitiveKind::Ritchie(RitchieItemKind::Fn);
    pub const GN: Self = MajorFugitiveKind::Ritchie(RitchieItemKind::Gn);
    pub const VN: Self = MajorFugitiveKind::Ritchie(RitchieItemKind::Vn);
    pub const PN: Self = MajorFugitiveKind::Ritchie(RitchieItemKind::Pn);
    pub const QN: Self = MajorFugitiveKind::Ritchie(RitchieItemKind::Qn);
    pub const TN: Self = MajorFugitiveKind::Ritchie(RitchieItemKind::Tn);
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityKind {
    Module,
    MajorItem {
        module_item_kind: MajorItemKind,
        connection: MajorItemConnectionKind,
    },
    AssocItem {
        assoc_item_kind: AssocItemKind,
    },
    TypeVariant,
    ImplBlock,
    Attr,
}

#[cfg(feature = "protocol_support")]
impl EntityKind {
    pub fn class(self) -> EntityClass {
        match self {
            EntityKind::Module => EntityClass::Module,
            EntityKind::MajorItem {
                module_item_kind, ..
            } => match module_item_kind {
                MajorItemKind::Type(_) => EntityClass::Type,
                MajorItemKind::Fugitive(major_fugitive_kind) => match major_fugitive_kind {
                    MajorFugitiveKind::Ritchie(_) => EntityClass::MajorRitchie,
                    MajorFugitiveKind::TypeAlias => EntityClass::TypeAlias,
                    MajorFugitiveKind::Val => EntityClass::Val,
                    MajorFugitiveKind::Formal => EntityClass::Formal,
                    MajorFugitiveKind::Const => EntityClass::Const,
                },
                MajorItemKind::Trait => EntityClass::Trait,
            },
            EntityKind::AssocItem { assoc_item_kind } => match assoc_item_kind {
                AssocItemKind::TypeItem(ty_item_kind) => ty_item_kind.into(),
                AssocItemKind::TraitItem(trai_item_kind)
                | AssocItemKind::TraitForTypeItem(trai_item_kind) => trai_item_kind.into(),
            },
            EntityKind::TypeVariant => EntityClass::TypeVariant,
            EntityKind::ImplBlock => EntityClass::ImplBlock,
            EntityKind::Attr => EntityClass::Attr,
        }
    }
}

#[salsa::debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MajorItemKind {
    Type(TypeKind),
    Fugitive(MajorFugitiveKind),
    Trait,
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssocItemKind {
    TypeItem(TypeItemKind),
    TraitItem(TraitItemKind),
    TraitForTypeItem(TraitItemKind),
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeItemKind {
    AssocVal,
    AssocRitchie(RitchieItemKind),
    AssocType,
    AssocFormal,
    AssocConst,
    MemoizedField,
    MethodRitchie(RitchieItemKind),
}

impl TypeItemKind {
    pub const ASSOC_VAL: Self = TypeItemKind::AssocVal;

    // # associated ritchie
    pub const ASSOC_FN: Self = TypeItemKind::AssocRitchie(RitchieItemKind::Fn);
    pub const ASSOC_GN: Self = TypeItemKind::AssocRitchie(RitchieItemKind::Gn);
    pub const ASSOC_VN: Self = TypeItemKind::AssocRitchie(RitchieItemKind::Vn);
    pub const ASSOC_PN: Self = TypeItemKind::AssocRitchie(RitchieItemKind::Pn);
    pub const ASSOC_QN: Self = TypeItemKind::AssocRitchie(RitchieItemKind::Qn);
    pub const ASSOC_TN: Self = TypeItemKind::AssocRitchie(RitchieItemKind::Tn);

    // # associated formal
    pub const ASSOC_FORMAL: Self = TypeItemKind::AssocFormal;

    // # method ritchie

    pub const METHOD_FN: Self = TypeItemKind::MethodRitchie(RitchieItemKind::Fn);
    pub const METHOD_GN: Self = TypeItemKind::MethodRitchie(RitchieItemKind::Gn);
    pub const METHOD_VN: Self = TypeItemKind::MethodRitchie(RitchieItemKind::Vn);
    pub const METHOD_PN: Self = TypeItemKind::MethodRitchie(RitchieItemKind::Pn);
    pub const METHOD_QN: Self = TypeItemKind::MethodRitchie(RitchieItemKind::Qn);
    pub const METHOD_TN: Self = TypeItemKind::MethodRitchie(RitchieItemKind::Tn);
}

#[cfg(feature = "protocol_support")]
impl Into<EntityClass> for TypeItemKind {
    fn into(self) -> EntityClass {
        match self {
            TypeItemKind::MemoizedField => EntityClass::MemoizedField,
            TypeItemKind::MethodRitchie(_) => EntityClass::MethodRitchie,
            TypeItemKind::AssocVal => EntityClass::AssocVal,
            TypeItemKind::AssocType => EntityClass::AssocType,
            TypeItemKind::AssocRitchie(_) => EntityClass::AssocRitchie,
            TypeItemKind::AssocFormal => EntityClass::AssocFormal,
            TypeItemKind::AssocConst => EntityClass::Const,
        }
    }
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraitItemKind {
    AssocRitchie(RitchieItemKind),
    AssocType,
    AssocVal,
    AssocFormal,
    AssocConst,
    MemoizedField,
    MethodRitchie(RitchieItemKind),
}

impl TraitItemKind {
    pub const ASSOC_VAL: Self = TraitItemKind::AssocVal;

    // # associated ritchie
    pub const ASSOC_FN: Self = TraitItemKind::AssocRitchie(RitchieItemKind::Fn);
    pub const ASSOC_GN: Self = TraitItemKind::AssocRitchie(RitchieItemKind::Gn);
    pub const ASSOC_VN: Self = TraitItemKind::AssocRitchie(RitchieItemKind::Vn);
    pub const ASSOC_PN: Self = TraitItemKind::AssocRitchie(RitchieItemKind::Pn);
    pub const ASSOC_QN: Self = TraitItemKind::AssocRitchie(RitchieItemKind::Qn);
    pub const ASSOC_TN: Self = TraitItemKind::AssocRitchie(RitchieItemKind::Tn);

    // # associated formal
    pub const ASSOC_FORMAL: Self = TraitItemKind::AssocFormal;

    // # method ritchie

    pub const METHOD_FN: Self = TraitItemKind::MethodRitchie(RitchieItemKind::Fn);
    pub const METHOD_GN: Self = TraitItemKind::MethodRitchie(RitchieItemKind::Gn);
    pub const METHOD_VN: Self = TraitItemKind::MethodRitchie(RitchieItemKind::Vn);
    pub const METHOD_PN: Self = TraitItemKind::MethodRitchie(RitchieItemKind::Pn);
    pub const METHOD_QN: Self = TraitItemKind::MethodRitchie(RitchieItemKind::Qn);
    pub const METHOD_TN: Self = TraitItemKind::MethodRitchie(RitchieItemKind::Tn);
}

#[cfg(feature = "protocol_support")]
impl Into<EntityClass> for TraitItemKind {
    fn into(self) -> EntityClass {
        match self {
            TraitItemKind::MemoizedField => EntityClass::MemoizedField,
            TraitItemKind::MethodRitchie(_) => EntityClass::MethodRitchie,
            TraitItemKind::AssocType => EntityClass::AssocType,
            TraitItemKind::AssocVal => EntityClass::AssocVal,
            TraitItemKind::AssocRitchie(_) => EntityClass::AssocRitchie,
            TraitItemKind::AssocFormal => EntityClass::AssocFormal,
            TraitItemKind::AssocConst => EntityClass::Const,
        }
    }
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MajorItemConnectionKind {
    Connected,
    Disconnected,
}
