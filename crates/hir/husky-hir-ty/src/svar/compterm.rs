use super::*;
use husky_entity_path::path::major_item::ty::TypePath;

/// a constant value, not a type
#[salsa::interned(db = HirTypeDb, jar = HirTypeJar, constructor = pub(crate) new)]
pub struct HirComptermTemplateVariable {
    pub ty: HirType,
    pub index: HirComptermTemplateVariableIndex,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirComptermTemplateVariableIndex {
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

impl HirComptermTemplateVariableIndex {
    pub fn attrs(self) -> HirTemplateVariableAttrs {
        match self {
            HirComptermTemplateVariableIndex::PathLeading { attrs, .. }
            | HirComptermTemplateVariableIndex::Other { attrs, .. } => attrs,
        }
    }

    pub fn class(self) -> HirTemplateVariableClass {
        self.attrs().class
    }
}
