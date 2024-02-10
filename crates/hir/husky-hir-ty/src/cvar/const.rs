use super::*;

#[salsa::interned(db = HirTypeDb, jar = HirTypeJar, constructor = pub(crate) new)]
pub struct HirConstSvar {
    pub ty: HirType,
    pub index: HirConstSymbolIndex,
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirConstSymbolIndex {
    PathLeading {
        attrs: HirTemplateVarAttrs,
        disambiguator: u8,
        ty_path: TypePath,
    },
    Other {
        attrs: HirTemplateVarAttrs,
        disambiguator: u8,
    },
}

impl HirConstSymbolIndex {
    pub fn attrs(self) -> HirTemplateVarAttrs {
        match self {
            HirConstSymbolIndex::PathLeading { attrs, .. }
            | HirConstSymbolIndex::Other { attrs, .. } => attrs,
        }
    }

    pub fn class(self) -> HirTemplateVarClass {
        self.attrs().class
    }
}
