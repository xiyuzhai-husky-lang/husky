use husky_coword::Ident;
use husky_entity_path::*;

use husky_vfs::Toolchain;
use salsa::DisplayWithDb;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = TermPreludeDb, jar = EtherealTermJar)]
pub enum TermEntityPath {
    Fugitive(FugitivePath),
    Trait(TraitPath),
    TypeOntology(TypePath),
    TypeInstance(TypePath),
    TypeVariant(TypeVariantPath),
}

#[test]
fn term_item_path_size_works() {
    assert_eq!(
        std::mem::size_of::<TermEntityPath>(),
        std::mem::size_of::<usize>()
    );
}

impl TermEntityPath {
    pub fn ty_ontology(self) -> Option<TypePath> {
        match self {
            TermEntityPath::TypeOntology(path) => Some(path),
            TermEntityPath::Fugitive(_)
            | TermEntityPath::Trait(_)
            | TermEntityPath::TypeInstance(_)
            | TermEntityPath::TypeVariant(_) => None,
        }
    }

    pub fn toolchain(self, db: &dyn TermPreludeDb) -> Toolchain {
        match self {
            TermEntityPath::Fugitive(path) => path.toolchain(db),
            TermEntityPath::Trait(path) => path.toolchain(db),
            TermEntityPath::TypeOntology(path) => path.toolchain(db),
            TermEntityPath::TypeInstance(path) => path.toolchain(db),
            TermEntityPath::TypeVariant(path) => path.toolchain(db),
        }
    }

    pub fn ident(self, db: &dyn TermPreludeDb) -> Ident {
        match self {
            TermEntityPath::Fugitive(path) => path.ident(db),
            TermEntityPath::Trait(path) => path.ident(db),
            TermEntityPath::TypeOntology(path) => path.ident(db),
            TermEntityPath::TypeInstance(path) => path.ident(db),
            TermEntityPath::TypeVariant(path) => path.ident(db),
        }
    }
}

impl From<FugitivePath> for TermEntityPath {
    fn from(value: FugitivePath) -> Self {
        TermEntityPath::Fugitive(value)
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
        f.write_str(self.ident(db).data(db))
    }
}

impl<Db: ?Sized + TermPreludeDb> DisplayWithDb<Db> for TermEntityPath {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        match self {
            TermEntityPath::Fugitive(path) => {
                f.write_str("Form(")?;
                path.display_with_db_fmt(f, db, salsa::DisplayFormatLevel::root())?;
                f.write_str(")")
            }
            TermEntityPath::Trait(path) => {
                f.write_str("Trait(")?;
                path.display_with_db_fmt(f, db, salsa::DisplayFormatLevel::root())?;
                f.write_str(")")
            }
            TermEntityPath::TypeOntology(path) => {
                f.write_str("TypeOntology(")?;
                path.display_with_db_fmt(f, db, salsa::DisplayFormatLevel::root())?;
                f.write_str(")")
            }
            TermEntityPath::TypeInstance(path) => {
                f.write_str("TypeConstructor(")?;
                path.display_with_db_fmt(f, db, salsa::DisplayFormatLevel::root())?;
                f.write_str(")")
            }
            TermEntityPath::TypeVariant(path) => {
                f.write_str("TypeVariant(")?;
                path.display_with_db_fmt(f, db, salsa::DisplayFormatLevel::root())?;
                f.write_str(")")
            }
        }
    }
}
