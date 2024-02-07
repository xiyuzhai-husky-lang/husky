#[cfg(feature = "protocol_support")]
use husky_entity_protocol::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeKind {
    Enum,
    Inductive,
    Record,
    Struct,
    Structure,
    Extern,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FugitiveKind {
    FunctionFn,
    FunctionGn,
    TypeAlias,
    Val,
    Formal,
    Const,
}

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
                MajorItemKind::Fugitive(fugitive_kind) => match fugitive_kind {
                    FugitiveKind::FunctionFn => EntityClass::FunctionFn,
                    FugitiveKind::FunctionGn => EntityClass::FunctionGn,
                    FugitiveKind::TypeAlias => EntityClass::TypeAlias,
                    FugitiveKind::Val => EntityClass::Val,
                    FugitiveKind::Formal => EntityClass::Formal,
                    FugitiveKind::Const => EntityClass::Const,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
pub enum MajorItemKind {
    Type(TypeKind),
    Fugitive(FugitiveKind),
    Trait,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssocItemKind {
    TypeItem(TypeItemKind),
    TraitItem(TraitItemKind),
    TraitForTypeItem(TraitItemKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeItemKind {
    MethodFn,
    AssocFunctionFn,
    AssocVal,
    AssocType,
    MemoizedField,
    AssocFunctionGn,
    AssocFormal,
    AssocConst,
}

impl Into<EntityClass> for TypeItemKind {
    fn into(self) -> EntityClass {
        match self {
            TypeItemKind::MemoizedField => EntityClass::MemoizedField,
            TypeItemKind::MethodFn => EntityClass::MethodFn,
            TypeItemKind::AssocVal => EntityClass::AssocVal,
            TypeItemKind::AssocType => EntityClass::AssocType,
            TypeItemKind::AssocFunctionFn => EntityClass::AssocFunctionFn,
            TypeItemKind::AssocFunctionGn => EntityClass::AssocFunctionGn,
            TypeItemKind::AssocFormal => EntityClass::AssocFormal,
            TypeItemKind::AssocConst => EntityClass::Const,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraitItemKind {
    MemoizedField,
    MethodFn,
    AssocType,
    AssocVal,
    AssocFunctionFn,
    AssocFunctionGn,
    AssocFormal,
    AssocConst,
}

impl Into<EntityClass> for TraitItemKind {
    fn into(self) -> EntityClass {
        match self {
            TraitItemKind::MemoizedField => EntityClass::MemoizedField,
            TraitItemKind::MethodFn => EntityClass::MethodFn,
            TraitItemKind::AssocType => EntityClass::AssocType,
            TraitItemKind::AssocVal => EntityClass::AssocVal,
            TraitItemKind::AssocFunctionFn => EntityClass::AssocFunctionFn,
            TraitItemKind::AssocFunctionGn => EntityClass::AssocFunctionGn,
            TraitItemKind::AssocFormal => EntityClass::AssocFormal,
            TraitItemKind::AssocConst => EntityClass::Const,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MajorItemConnectionKind {
    Connected,
    Disconnected,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnumVariantKind {
    Constant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutineKind {
    Normal,
    TypeCall,
    TypeAssoc,
    TraitAssoc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawMembRoutineKind {
    Proc,
    Func,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemberKind {
    Field,
    Method { is_lazy: bool },
    Call,
    TraitAssocType,
    TraitAssocConstSize,
    TraitAssocAny,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FieldKind {
    StructRegular,
    StructDefault,
    StructDerived,
    StructMemo, // property is not store along with struct
    RecordRegular,
    RecordProperty,
}
