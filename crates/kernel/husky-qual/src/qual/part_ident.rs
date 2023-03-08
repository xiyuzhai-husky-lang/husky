use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PartIdent {
    Field(Ident),
    TupleIndex(u8),
    ListIndex(u8),
}
