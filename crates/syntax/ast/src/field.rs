use crate::*;
use word::Paradigm;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FieldKind {
    StructOriginal,
    StructDefault { default: RawExprIdx },
    StructDerivedEager { derivation: RawExprIdx },
    StructDerivedLazy { paradigm: Paradigm },
    RecordOriginal,
    RecordDerived,
}
