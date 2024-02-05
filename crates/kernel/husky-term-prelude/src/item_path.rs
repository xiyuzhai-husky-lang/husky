use husky_coword::Ident;
use husky_entity_path::*;

use husky_vfs::Toolchain;
use salsa::DisplayWithDb;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
pub enum ItemPathTerm {
    MajorFugitive(MajorFugitivePath),
    Trait(TraitPath),
    TypeOntology(TypePath),
    TypeInstance(TypePath),
    TypeVariant(TypeVariantPath),
}

#[test]
fn term_item_path_size_works() {
    assert_eq!(
        std::mem::size_of::<ItemPathTerm>(),
        std::mem::size_of::<usize>()
    );
}

impl ItemPathTerm {
    pub fn ty_ontology(self) -> Option<TypePath> {
        match self {
            ItemPathTerm::TypeOntology(path) => Some(path),
            ItemPathTerm::MajorFugitive(_)
            | ItemPathTerm::Trait(_)
            | ItemPathTerm::TypeInstance(_)
            | ItemPathTerm::TypeVariant(_) => None,
        }
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        match self {
            ItemPathTerm::MajorFugitive(path) => path.toolchain(db),
            ItemPathTerm::Trait(path) => path.toolchain(db),
            ItemPathTerm::TypeOntology(path) => path.toolchain(db),
            ItemPathTerm::TypeInstance(path) => path.toolchain(db),
            ItemPathTerm::TypeVariant(path) => path.toolchain(db),
        }
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        match self {
            ItemPathTerm::MajorFugitive(path) => path.ident(db),
            ItemPathTerm::Trait(path) => path.ident(db),
            ItemPathTerm::TypeOntology(path) => path.ident(db),
            ItemPathTerm::TypeInstance(path) => path.ident(db),
            ItemPathTerm::TypeVariant(path) => path.ident(db),
        }
    }
}

impl From<MajorFugitivePath> for ItemPathTerm {
    fn from(value: MajorFugitivePath) -> Self {
        ItemPathTerm::MajorFugitive(value)
    }
}

impl From<TraitPath> for ItemPathTerm {
    fn from(value: TraitPath) -> Self {
        ItemPathTerm::Trait(value)
    }
}

impl DisplayWithDb for ItemPathTerm {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        f.write_str(self.ident(db).data(db))
    }
}
