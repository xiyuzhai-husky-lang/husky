use husky_entity_path::*;
use salsa::DisplayWithDb;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum DecItemPath {
    Fugitive(MajorFugitivePath),
    Trait(TraitPath),
    Type(TypePath),
    TypeVariant(TypeVariantPath),
}

impl From<ItemPathTerm> for DecItemPath {
    fn from(path: ItemPathTerm) -> Self {
        match path {
            ItemPathTerm::Fugitive(path) => DecItemPath::Fugitive(path),
            ItemPathTerm::Trait(path) => DecItemPath::Trait(path),
            ItemPathTerm::TypeOntology(path) | ItemPathTerm::TypeInstance(path) => {
                DecItemPath::Type(path)
            }
            ItemPathTerm::TypeVariant(path) => DecItemPath::TypeVariant(path),
        }
    }
}

impl From<ItemPathTerm> for DecTerm {
    fn from(path: ItemPathTerm) -> Self {
        DecTerm::EntityPath(path.into())
    }
}

impl DecItemPath {
    pub fn ty_path(self) -> Option<TypePath> {
        match self {
            DecItemPath::Type(path) => Some(path),
            DecItemPath::Fugitive(_) | DecItemPath::Trait(_) | DecItemPath::TypeVariant(_) => None,
        }
    }
}
impl From<MajorFugitivePath> for DecTerm {
    fn from(value: MajorFugitivePath) -> Self {
        DecTerm::EntityPath(value.into())
    }
}

impl From<TraitPath> for DecTerm {
    fn from(value: TraitPath) -> Self {
        DecTerm::EntityPath(value.into())
    }
}

impl From<TypePath> for DecTerm {
    fn from(value: TypePath) -> Self {
        DecTerm::EntityPath(value.into())
    }
}

impl DecItemPath {
    #[inline(never)]
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        _ctx: &SymbolDecTermNameMap,
    ) -> std::fmt::Result {
        match self {
            DecItemPath::Fugitive(path) => path.display_fmt_with_db(f, db),
            DecItemPath::Trait(path) => path.display_fmt_with_db(f, db),
            DecItemPath::Type(path) => path.display_fmt_with_db(f, db),
            DecItemPath::TypeVariant(path) => path.display_fmt_with_db(f, db),
        }
    }
}
