use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum HirSymbol {
    Type(HirTypeSymbol),
    Const(HirConstSymbol),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HirTypeSymbol(u8);

#[salsa::interned(db = HirTypeDb, jar = HirTypeJar)]
pub struct HirConstSymbol {
    pub ty: HirType,
    pub index: SymbolIndex,
}
