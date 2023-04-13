use super::*;
use vec_like::{AsVecMapEntry, VecMap};

#[derive(Debug, Default, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct SolidTerms {
    entries: Vec<SolidTermEntry>,
}

impl SolidTerms {
    fn intern() -> SolidTerm {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct SolidTermEntry {
    src: SolidTermSource,
    data: SolidTermData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct SolidTerm(u32);

impl SolidTerm {
    pub(crate) fn new(solid_terms: &mut SolidTerms, data: SolidTermData) -> Self {
        Self(todo!())
    }

    pub(crate) fn data(self, solid_terms: &SolidTerms) -> &SolidTermData {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
// #[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub enum SolidTermSource {}
