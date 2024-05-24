pub mod ritchie;

use self::ritchie::*;
#[cfg(feature = "protocol_support")]
use husky_entity_protocol::*;
use serde::{Deserialize, Serialize};

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeKind {
    Enum,
    Inductive,
    Record,
    Struct,
    Structure,
    Extern,
}

#[enum_class::from_variants]
#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MajorFormKind {
    Ritchie(RitchieItemKind),
    TypeAlias,
    Val,
    Static,
    Compterm,
    Conceptual,
}

impl MajorFormKind {
    pub const FN: Self = MajorFormKind::Ritchie(RitchieItemKind::Fn);
    pub const GN: Self = MajorFormKind::Ritchie(RitchieItemKind::Gn);
    pub const VN: Self = MajorFormKind::Ritchie(RitchieItemKind::Vn);
    pub const PN: Self = MajorFormKind::Ritchie(RitchieItemKind::Pn);
    pub const QN: Self = MajorFormKind::Ritchie(RitchieItemKind::Qn);
    pub const BN: Self = MajorFormKind::Ritchie(RitchieItemKind::Bn);
    pub const SN: Self = MajorFormKind::Ritchie(RitchieItemKind::Sn);
    pub const TN: Self = MajorFormKind::Ritchie(RitchieItemKind::Tn);
}

impl MajorFormKind {
    #[track_caller]
    pub fn ritchie(self) -> RitchieItemKind {
        match self {
            MajorFormKind::Ritchie(slf) => slf,
            _ => unreachable!(),
        }
    }
}

#[salsa::derive_debug_with_db]
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
    Script,
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
                MajorItemKind::Form(major_form_kind) => match major_form_kind {
                    MajorFormKind::Ritchie(_) => EntityClass::MajorFunctionRitchie,
                    MajorFormKind::TypeAlias => EntityClass::TypeAlias,
                    MajorFormKind::Val => EntityClass::Val,
                    MajorFormKind::Conceptual => EntityClass::Formal,
                    MajorFormKind::Compterm => EntityClass::Compterm,
                    MajorFormKind::Static => EntityClass::Static,
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
            EntityKind::Script => EntityClass::Script,
        }
    }
}

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MajorItemKind {
    Type(TypeKind),
    Form(MajorFormKind),
    Trait,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssocItemKind {
    TypeItem(TypeItemKind),
    TraitItem(TraitItemKind),
    TraitForTypeItem(TraitItemKind),
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeItemKind {
    AssocVal,
    AssocRitchie(RitchieItemKind),
    AssocType,
    AssocConceptual,
    AssocStatic,
    AssocTermic,
    MemoizedField,
    MethodRitchie(RitchieItemKind),
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
            TypeItemKind::AssocConceptual => EntityClass::AssocDef,
            TypeItemKind::AssocStatic => EntityClass::Static,
            TypeItemKind::AssocTermic => EntityClass::Compterm,
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraitItemKind {
    AssocRitchie(RitchieItemKind),
    AssocType,
    AssocVal,
    AssocConceptual,
    AssocStatic,
    AssocTermic,
    MemoizedField,
    MethodRitchie(RitchieItemKind),
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
            TraitItemKind::AssocConceptual => EntityClass::AssocDef,
            TraitItemKind::AssocStatic => EntityClass::Static,
            TraitItemKind::AssocTermic => EntityClass::Compterm,
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MajorItemConnectionKind {
    Connected,
    Disconnected,
}
