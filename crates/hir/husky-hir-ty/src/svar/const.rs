use super::*;

/// a constant value, not a type
#[salsa::interned(db = HirTypeDb, jar = HirTypeJar, constructor = pub(crate) new)]
pub struct HirConstSvar {
    pub ty: HirType,
    pub index: HirConstSvarIndex,
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirConstSvarIndex {
    PathLeading {
        attrs: HirTemplateSvarAttrs,
        disambiguator: u8,
        ty_path: TypePath,
    },
    Other {
        attrs: HirTemplateSvarAttrs,
        disambiguator: u8,
    },
}

impl HirConstSvarIndex {
    pub fn attrs(self) -> HirTemplateSvarAttrs {
        match self {
            HirConstSvarIndex::PathLeading { attrs, .. }
            | HirConstSvarIndex::Other { attrs, .. } => attrs,
        }
    }

    pub fn class(self) -> HirTemplateSvarClass {
        self.attrs().class
    }
}
