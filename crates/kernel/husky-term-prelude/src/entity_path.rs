use husky_entity_path::*;
use husky_vfs::VfsPathMenu;
use salsa::DisplayWithDb;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = TermPreludeDb, jar = TermJar)]
pub enum TermEntityPath {
    Form(FormPath),
    Trait(TraitPath),
    TypeOntology(TypePath),
    TypeConstructor(TypePath),
}

impl TermEntityPath {
    pub fn ty_ontology(self) -> Option<TypePath> {
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

impl From<TraitPath> for TermEntityPath {
    fn from(value: TraitPath) -> Self {
        TermEntityPath::Trait(value)
    }
}

impl TermEntityPath {
    pub fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermPreludeDb,
    ) -> std::fmt::Result {
        self.display_with_db_fmt(f, db, salsa::DisplayFormatLevel::root())
    }
}

impl<Db: ?Sized + TermPreludeDb> DisplayWithDb<Db> for TermEntityPath {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        match self {
            TermEntityPath::Form(path) => {
                path.display_with_db_fmt(f, db, salsa::DisplayFormatLevel::root())
            }
            TermEntityPath::Trait(path) => {
                path.display_with_db_fmt(f, db, salsa::DisplayFormatLevel::root())
            }
            TermEntityPath::TypeOntology(path) => {
                path.display_with_db_fmt(f, db, salsa::DisplayFormatLevel::root())
            }
            TermEntityPath::TypeConstructor(path) => {
                path.display_with_db_fmt(f, db, salsa::DisplayFormatLevel::root())
            }
        }
    }
}
