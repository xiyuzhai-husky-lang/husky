use husky_entity_path::*;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = TermDb, jar = TermJar)]
pub enum TermEntityPath {
    Form(FormPath),
    Trait(TraitPath),
    TypeOntology(TypePath),
    TypeConstructor(TypePath),
}

impl TermEntityPath {
    pub fn ty_ontology_path(self) -> Option<TypePath> {
        match self {
            TermEntityPath::TypeOntology(path) => Some(path),
            TermEntityPath::Form(_)
            | TermEntityPath::Trait(_)
            | TermEntityPath::TypeConstructor(_) => None,
        }
    }
}

impl From<FormPath> for TermEntityPath {
    fn from(value: FormPath) -> Self {
        TermEntityPath::Form(value)
    }
}

impl From<FormPath> for Term {
    fn from(value: FormPath) -> Self {
        Term::EntityPath(value.into())
    }
}

impl From<TraitPath> for TermEntityPath {
    fn from(value: TraitPath) -> Self {
        TermEntityPath::Trait(value)
    }
}

impl From<TraitPath> for Term {
    fn from(value: TraitPath) -> Self {
        Term::EntityPath(value.into())
    }
}

impl TermEntityPath {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        // .display_with_db_fmt(f, db, salsa::DisplayFormatLevel::root())
        todo!()
    }
}
