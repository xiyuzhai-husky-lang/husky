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
    pub fn protocol(self) -> EntityKindProtocol {
        match self {
            EntityKind::Module => EntityKindProtocol::Module,
            EntityKind::MajorItem {
                module_item_kind, ..
            } => match module_item_kind {
                MajorItemKind::Type(_) => EntityKindProtocol::Type,
                MajorItemKind::Fugitive(fugitive_kind) => match fugitive_kind {
                    FugitiveKind::FunctionFn => EntityKindProtocol::FunctionFn,
                    FugitiveKind::FunctionGn => EntityKindProtocol::FunctionGn,
                    FugitiveKind::AliasType => EntityKindProtocol::AliasType,
                    FugitiveKind::Val => EntityKindProtocol::Val,
                },
                MajorItemKind::Trait => EntityKindProtocol::Trait,
            },
            EntityKind::AssociatedItem {
                associated_item_kind,
            } => match associated_item_kind {
                AssociatedItemKind::TraitItem(trai_item_kind) => match trai_item_kind {
                    TraitItemKind::MethodFn => EntityKindProtocol::MethodFn,
                    TraitItemKind::AssociatedType => EntityKindProtocol::AssociatedType,
                },
                AssociatedItemKind::TypeItem(ty_item_kind) => match ty_item_kind {
                    TypeItemKind::MethodFn => EntityKindProtocol::MethodFn,
                    TypeItemKind::AssociatedFunctionFn => EntityKindProtocol::AssociatedFunctionFn,
                    TypeItemKind::AssociatedVal => EntityKindProtocol::AssociatedVal,
                    TypeItemKind::AssociatedType => EntityKindProtocol::AssociatedType,
                    TypeItemKind::MemoizedField => EntityKindProtocol::MemoizedField,
                },
                AssociatedItemKind::TraitForTypeItem(trai_for_ty_item_kind) => {
                    match trai_for_ty_item_kind {
                        TraitItemKind::MethodFn => EntityKindProtocol::MethodFn,
                        TraitItemKind::AssociatedType => EntityKindProtocol::AssociatedType,
                    }
                }
            },
            EntityKind::TypeVariant => EntityKindProtocol::TypeVariant,
            EntityKind::ImplBlock => EntityKindProtocol::ImplBlock,
            EntityKind::Attr => EntityKindProtocol::Attr,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraitItemKind {
    MethodFn,
    AssociatedType,
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
