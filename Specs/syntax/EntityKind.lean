-- chore
pub enum TyKind {
    Enum,
    Record,
    Struct,
    Primitive,
    Vec,
    Slice,
    CyclicSlice,
    Array,
    Tuple,
    Mor,
    ThickFp,
    AssociatedAny,
    ThisAny,
    TargetOutputAny,
    SpatialPlaceholderAny,
    BoxAny,
    HigherKind,
    Ref,
    Option,
}

impl From<TyKeyword> for TyKind {
    fn from(keyword: TyKeyword) -> Self {
        match keyword {
            TyKeyword::Struct => TyKind::Struct,
            TyKeyword::Enum => TyKind::Enum,
            TyKeyword::Record => TyKind::Record,
        }
    }
}

pub enum MemberKind {
    Field,
    Method { is_lazy: bool },
    Call,
    TraitAssociatedType,
    TraitAssociatedConstSize,
    TraitAssociatedAny,
}

pub enum EntityKind {
    Module,
    Type(TyKind),
    Trait,
    Member(MemberKind),
    Function { requires_lazy: bool },
    Feature,
    EnumVariant,
    Main,
}