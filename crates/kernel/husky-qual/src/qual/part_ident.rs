use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PartIdent {
    Field(Identifier),
    TupleIndex(u8),
    ListIndex(u8),
}
