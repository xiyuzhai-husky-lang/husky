use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct SolidTerms {}

impl SolidTerms {
    pub(crate) fn intern() -> SolidTerm {
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
pub struct SolidTerm {}

impl SolidTerm {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
// #[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub enum SolidTermSource {}
