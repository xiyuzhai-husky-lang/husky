use husky_token::ParadigmKeyword;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AstFieldKind {
    StructOriginal,
    StructDefault { default: ExprIdx },
    StructDerivedEager { derivation: ExprIdx },
    StructProperty { paradigm: ParadigmKeyword },
    RecordOriginal,
    RecordDerived,
}

impl Into<FieldKind> for AstFieldKind {
    fn into(self) -> FieldKind {
        match self {
            AstFieldKind::StructOriginal => FieldKind::StructRegular,
            AstFieldKind::StructDefault { .. } => FieldKind::StructDefault,
            AstFieldKind::StructDerivedEager { .. } => FieldKind::StructDerived,
            AstFieldKind::StructProperty { .. } => FieldKind::StructMemo,
            AstFieldKind::RecordOriginal => FieldKind::RecordRegular,
            AstFieldKind::RecordDerived => FieldKind::RecordProperty,
        }
    }
}
