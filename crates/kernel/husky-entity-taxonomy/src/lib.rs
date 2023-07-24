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
    Fn,
    Gn,
    AliasType,
    Val,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityKind {
    Module,
    ModuleItem {
        module_item_kind: ModuleItemKind,
        connection: ModuleItemConnectionKind,
    },
    AssociatedItem {
        associated_item_kind: AssociatedItemKind,
    },
    TypeVariant,
    Trait,
    ImplBlock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
pub enum ModuleItemKind {
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
    AssociatedFn,
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
pub enum ModuleItemConnectionKind {
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
