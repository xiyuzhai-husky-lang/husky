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
    AliasType,
    Val,
    Formal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityKind {
    Module,
    MajorItem {
        module_item_kind: MajorItemKind,
        connection: MajorItemConnectionKind,
    },
    AssociatedItem {
        associated_item_kind: AssociatedItemKind,
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
                    FugitiveKind::AliasType => EntityClass::AliasType,
                    FugitiveKind::Val => EntityClass::Val,
                    FugitiveKind::Formal => EntityClass::Formal,
                },
                MajorItemKind::Trait => EntityClass::Trait,
            },
            EntityKind::AssociatedItem {
                associated_item_kind,
            } => match associated_item_kind {
                AssociatedItemKind::TraitItem(trai_item_kind) => match trai_item_kind {
                    TraitItemKind::MethodFn => EntityClass::MethodFn,
                    TraitItemKind::AssociatedType => EntityClass::AssociatedType,
                    TraitItemKind::AssociatedVal => EntityClass::AssociatedVal,
                    TraitItemKind::AssociatedFunctionFn => EntityClass::AssociatedFunctionFn,
                    TraitItemKind::AssociatedFunctionGn => EntityClass::AssociatedFunctionGn,
                },
                AssociatedItemKind::TypeItem(ty_item_kind) => match ty_item_kind {
                    TypeItemKind::MethodFn => EntityClass::MethodFn,
                    TypeItemKind::AssociatedVal => EntityClass::AssociatedVal,
                    TypeItemKind::AssociatedType => EntityClass::AssociatedType,
                    TypeItemKind::MemoizedField => EntityClass::MemoizedField,
                    TypeItemKind::AssociatedFunctionFn => EntityClass::AssociatedFunctionFn,
                    TypeItemKind::AssociatedFunctionGn => EntityClass::AssociatedFunctionGn,
                },
                AssociatedItemKind::TraitForTypeItem(trai_for_ty_item_kind) => {
                    match trai_for_ty_item_kind {
                        TraitItemKind::MethodFn => EntityClass::MethodFn,
                        TraitItemKind::AssociatedType => EntityClass::AssociatedType,
                        TraitItemKind::AssociatedVal => EntityClass::AssociatedVal,
                        TraitItemKind::AssociatedFunctionFn => EntityClass::AssociatedFunctionFn,
                        TraitItemKind::AssociatedFunctionGn => EntityClass::AssociatedFunctionGn,
                    }
                }
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
pub enum AssociatedItemKind {
    TraitItem(TraitItemKind),
    TypeItem(TypeItemKind),
    TraitForTypeItem(TraitItemKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeItemKind {
    MethodFn,
    AssociatedFunctionFn,
    AssociatedVal,
    AssociatedType,
    MemoizedField,
    AssociatedFunctionGn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraitItemKind {
    MethodFn,
    AssociatedType,
    AssociatedVal,
    AssociatedFunctionFn,
    AssociatedFunctionGn,
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
    TypeAssociated,
    TraitAssociated,
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
    TraitAssociatedType,
    TraitAssociatedConstSize,
    TraitAssociatedAny,
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
