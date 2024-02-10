use crate::*;

pub struct SymbolData {
    name: String,
    kind: SymbolKind,
}

pub enum SymbolKind {
    Operator,
    Literal,
    Variable,
}

pub struct SymbolId(ShiftedU32);

pub struct BcSymbolStorage {}

impl std::ops::Index<SymbolId> for BcSymbolStorage {
    type Output = SymbolData;

    fn index(&self, index: SymbolId) -> &Self::Output {
        todo!()
    }
}
