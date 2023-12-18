use super::*;

#[salsa::interned(db = HirTypeDb, jar = HirTypeJar, constructor = pub(crate) new)]
pub struct HirConstSymbol {
    pub ty: HirType,
    pub index: HirConstSymbolIndex,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirConstSymbolIndex {
    PathLeading {
        attrs: HirTemplateSymbolAttrs,
        disambiguator: u8,
        ty_path: TypePath,
    },
    Other {
        attrs: HirTemplateSymbolAttrs,
        disambiguator: u8,
    },
}

impl HirConstSymbolIndex {
    pub fn attrs(self) -> HirTemplateSymbolAttrs {
        match self {
            HirConstSymbolIndex::PathLeading { attrs, .. }
            | HirConstSymbolIndex::Other { attrs, .. } => attrs,
        }
    }

    pub fn class(self) -> HirTemplateSymbolClass {
        self.attrs().class
    }
}
