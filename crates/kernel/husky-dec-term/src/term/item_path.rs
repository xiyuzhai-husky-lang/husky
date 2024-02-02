use husky_entity_path::*;
use salsa::DisplayWithDb;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum ItemPathDeclarativeTerm {
    Fugitive(FugitivePath),
    Trait(TraitPath),
    Type(TypePath),
    TypeVariant(TypeVariantPath),
}

impl From<ItemPathTerm> for ItemPathDeclarativeTerm {
    fn from(path: ItemPathTerm) -> Self {
        match path {
            ItemPathTerm::Fugitive(path) => ItemPathDeclarativeTerm::Fugitive(path),
            ItemPathTerm::Trait(path) => ItemPathDeclarativeTerm::Trait(path),
            ItemPathTerm::TypeOntology(path) | ItemPathTerm::TypeInstance(path) => {
                ItemPathDeclarativeTerm::Type(path)
            }
            ItemPathTerm::TypeVariant(path) => ItemPathDeclarativeTerm::TypeVariant(path),
        }
    }
}

impl From<ItemPathTerm> for DeclarativeTerm {
    fn from(path: ItemPathTerm) -> Self {
        DeclarativeTerm::EntityPath(path.into())
    }
}

impl ItemPathDeclarativeTerm {
    pub fn ty_path(self) -> Option<TypePath> {
        match self {
            ItemPathDeclarativeTerm::Type(path) => Some(path),
            ItemPathDeclarativeTerm::Fugitive(_)
            | ItemPathDeclarativeTerm::Trait(_)
            | ItemPathDeclarativeTerm::TypeVariant(_) => None,
        }
    }
}
impl From<FugitivePath> for DeclarativeTerm {
    fn from(value: FugitivePath) -> Self {
        DeclarativeTerm::EntityPath(value.into())
    }
}

impl From<TraitPath> for DeclarativeTerm {
    fn from(value: TraitPath) -> Self {
        DeclarativeTerm::EntityPath(value.into())
    }
}

impl From<TypePath> for DeclarativeTerm {
    fn from(value: TypePath) -> Self {
        DeclarativeTerm::EntityPath(value.into())
    }
}

impl ItemPathDeclarativeTerm {
    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        _ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        match self {
            ItemPathDeclarativeTerm::Fugitive(path) => path.display_with_db_fmt(f, db),
            ItemPathDeclarativeTerm::Trait(path) => path.display_with_db_fmt(f, db),
            ItemPathDeclarativeTerm::Type(path) => path.display_with_db_fmt(f, db),
            ItemPathDeclarativeTerm::TypeVariant(path) => path.display_with_db_fmt(f, db),
        }
    }
}
