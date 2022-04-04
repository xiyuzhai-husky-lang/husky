use word::TyKeyword;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RawTyKind {
    Enum,
    Record,
    Struct,
    Primitive,
    Vec,
    Array,
    Other,
}

impl From<TyKeyword> for RawTyKind {
    fn from(keyword: TyKeyword) -> Self {
        match keyword {
            TyKeyword::Struct => RawTyKind::Struct,
            TyKeyword::Rename => todo!(),
            TyKeyword::Enum => RawTyKind::Enum,
            TyKeyword::Props => todo!(),
            TyKeyword::Record => RawTyKind::Record,
        }
    }
}
