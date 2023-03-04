use husky_entity_path::*;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = RawTermDb, jar = RawTermJar)]
pub enum RawTermEntityPath {
    Form(FormPath),
    Trait(TraitPath),
    TypeOntology(TypePath),
    TypeConstructor(TypePath),
}

impl RawTermEntityPath {
    pub fn ty_ontology_path(self) -> Option<TypePath> {
        match self {
            RawTermEntityPath::TypeOntology(path) => Some(path),
            RawTermEntityPath::Form(_)
            | RawTermEntityPath::Trait(_)
            | RawTermEntityPath::TypeConstructor(_) => None,
        }
    }
}

impl From<FormPath> for RawTermEntityPath {
    fn from(value: FormPath) -> Self {
        RawTermEntityPath::Form(value)
    }
}

impl From<FormPath> for RawTerm {
    fn from(value: FormPath) -> Self {
        RawTerm::EntityPath(value.into())
    }
}

impl From<TraitPath> for RawTermEntityPath {
    fn from(value: TraitPath) -> Self {
        RawTermEntityPath::Trait(value)
    }
}

impl From<TraitPath> for RawTerm {
    fn from(value: TraitPath) -> Self {
        RawTerm::EntityPath(value.into())
    }
}

impl RawTermEntityPath {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        // .display_with_db_fmt(f, db, salsa::DisplayFormatLevel::root())
        todo!()
    }
}
