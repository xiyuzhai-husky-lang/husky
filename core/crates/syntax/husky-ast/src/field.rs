use crate::*;
use word::Paradigm;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FieldAstKind {
    StructOriginal,
    StructDefault { default: RawExprIdx },
    StructDerivedEager { derivation: RawExprIdx },
    StructDerivedLazy { paradigm: Paradigm },
    RecordOriginal,
    RecordDerived,
}

impl Into<FieldKind> for FieldAstKind {
    fn into(self) -> FieldKind {
        match self {
            FieldAstKind::StructOriginal => FieldKind::StructOriginal,
            FieldAstKind::StructDefault { .. } => FieldKind::StructDefault,
            FieldAstKind::StructDerivedEager { .. } => FieldKind::StructDerivedEager,
            FieldAstKind::StructDerivedLazy { .. } => FieldKind::StructDerivedLazy,
            FieldAstKind::RecordOriginal => FieldKind::RecordOriginal,
            FieldAstKind::RecordDerived => FieldKind::RecordDerived,
        }
    }
}
