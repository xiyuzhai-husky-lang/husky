use super::*;

#[salsa::interned(db = HirTypeDb, jar = HirTypeJar, constructor = pub(crate) new)]
pub struct HirConstSymbol {
    pub ty: HirType,
    pub index: HirConstSymbolIndex,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirConstSymbolIndex {
    PathLeading {
        disambiguator: u8,
        ty_path: TypePath,
    },
    Other {
        disambiguator: u8,
    },
}
