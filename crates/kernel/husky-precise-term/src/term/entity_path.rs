use husky_entity_path::*;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = PreciseTermDb, jar = PreciseTermJar)]
pub enum PreciseTermEntityPath {
    Form(FormPath),
    Trait(TraitPath),
    TypeOntology(TypePath),
    TypeConstructor(TypePath),
}

impl PreciseTermEntityPath {
    pub fn ty_ontology_path(self) -> Option<TypePath> {
        match self {
            PreciseTermEntityPath::TypeOntology(path) => Some(path),
            PreciseTermEntityPath::Form(_)
            | PreciseTermEntityPath::Trait(_)
            | PreciseTermEntityPath::TypeConstructor(_) => None,
        }
    }
}

impl From<FormPath> for PreciseTermEntityPath {
    fn from(value: FormPath) -> Self {
        PreciseTermEntityPath::Form(value)
    }
}

impl From<FormPath> for PreciseTerm {
    fn from(value: FormPath) -> Self {
        PreciseTerm::EntityPath(value.into())
    }
}

impl From<TraitPath> for PreciseTermEntityPath {
    fn from(value: TraitPath) -> Self {
        PreciseTermEntityPath::Trait(value)
    }
}

impl From<TraitPath> for PreciseTerm {
    fn from(value: TraitPath) -> Self {
        PreciseTerm::EntityPath(value.into())
    }
}

impl PreciseTermEntityPath {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn PreciseTermDb,
        ctx: &mut PreciseTermShowContext,
    ) -> std::fmt::Result {
        // .display_with_db_fmt(f, db, salsa::DisplayFormatLevel::root())
        todo!()
    }
}
