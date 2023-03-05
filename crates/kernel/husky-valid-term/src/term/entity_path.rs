use husky_entity_path::*;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = ValidTermDb, jar = ValidTermJar)]
pub enum ValidTermEntityPath {
    Form(FormPath),
    Trait(TraitPath),
    TypeOntology(TypePath),
    TypeConstructor(TypePath),
}

impl ValidTermEntityPath {
    pub fn from_precise(db: &dyn PreciseTermDb, precise_term: PreciseTermEntityPath) -> Self {
        match precise_term {
            PreciseTermEntityPath::Form(path) => ValidTermEntityPath::Form(path),
            PreciseTermEntityPath::Trait(path) => ValidTermEntityPath::Trait(path),
            PreciseTermEntityPath::TypeOntology(path) => ValidTermEntityPath::TypeOntology(path),
            PreciseTermEntityPath::TypeConstructor(path) => {
                ValidTermEntityPath::TypeConstructor(path)
            }
        }
    }

    pub fn ty_ontology_path(self) -> Option<TypePath> {
        match self {
            ValidTermEntityPath::TypeOntology(path) => Some(path),
            ValidTermEntityPath::Form(_)
            | ValidTermEntityPath::Trait(_)
            | ValidTermEntityPath::TypeConstructor(_) => None,
        }
    }
}

impl From<FormPath> for ValidTermEntityPath {
    fn from(value: FormPath) -> Self {
        ValidTermEntityPath::Form(value)
    }
}

impl From<FormPath> for ValidTerm {
    fn from(value: FormPath) -> Self {
        ValidTerm::EntityPath(value.into())
    }
}

impl From<TraitPath> for ValidTermEntityPath {
    fn from(value: TraitPath) -> Self {
        ValidTermEntityPath::Trait(value)
    }
}

impl From<TraitPath> for ValidTerm {
    fn from(value: TraitPath) -> Self {
        ValidTerm::EntityPath(value.into())
    }
}

impl ValidTermEntityPath {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn ValidTermDb,
        ctx: &mut ValidTermShowContext,
    ) -> std::fmt::Result {
        // .display_with_db_fmt(f, db, salsa::DisplayFormatLevel::root())
        todo!()
    }
}
