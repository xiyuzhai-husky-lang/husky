use super::*;

/// a constant value, not a type
#[salsa::interned(db = HirTypeDb, jar = HirTypeJar, constructor = pub(crate) new)]
pub struct HirConstTemplateVariable {
    pub ty: HirType,
    pub index: HirConstTemplateVariableIndex,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirConstTemplateVariableIndex {
    PathLeading {
        attrs: HirTemplateVariableAttrs,
        disambiguator: u8,
        ty_path: TypePath,
    },
    Other {
        attrs: HirTemplateVariableAttrs,
        disambiguator: u8,
    },
}

impl HirConstTemplateVariableIndex {
    pub fn attrs(self) -> HirTemplateVariableAttrs {
        match self {
            HirConstTemplateVariableIndex::PathLeading { attrs, .. }
            | HirConstTemplateVariableIndex::Other { attrs, .. } => attrs,
        }
    }

    pub fn class(self) -> HirTemplateVariableClass {
        self.attrs().class
    }
}
