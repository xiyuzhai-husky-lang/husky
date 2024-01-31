use husky_entity_path::*;
use salsa::DisplayWithDb;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum EntityPathDeclarativeTerm {
    Fugitive(FugitivePath),
    Trait(TraitPath),
    Type(TypePath),
    TypeVariant(TypeVariantPath),
}

impl From<TermEntityPath> for EntityPathDeclarativeTerm {
    fn from(path: TermEntityPath) -> Self {
        match path {
            TermEntityPath::Fugitive(path) => EntityPathDeclarativeTerm::Fugitive(path),
            TermEntityPath::Trait(path) => EntityPathDeclarativeTerm::Trait(path),
            TermEntityPath::TypeOntology(path) | TermEntityPath::TypeInstance(path) => {
                EntityPathDeclarativeTerm::Type(path)
            }
            TermEntityPath::TypeVariant(path) => EntityPathDeclarativeTerm::TypeVariant(path),
        }
    }
}

impl From<TermEntityPath> for DeclarativeTerm {
    fn from(path: TermEntityPath) -> Self {
        DeclarativeTerm::EntityPath(path.into())
    }
}

impl EntityPathDeclarativeTerm {
    pub fn ty_path(self) -> Option<TypePath> {
        match self {
            EntityPathDeclarativeTerm::Type(path) => Some(path),
            EntityPathDeclarativeTerm::Fugitive(_)
            | EntityPathDeclarativeTerm::Trait(_)
            | EntityPathDeclarativeTerm::TypeVariant(_) => None,
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

impl EntityPathDeclarativeTerm {
    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        _ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        match self {
            EntityPathDeclarativeTerm::Fugitive(path) => path.display_with_db_fmt(f, db),
            EntityPathDeclarativeTerm::Trait(path) => path.display_with_db_fmt(f, db),
            EntityPathDeclarativeTerm::Type(path) => path.display_with_db_fmt(f, db),
            EntityPathDeclarativeTerm::TypeVariant(path) => path.display_with_db_fmt(f, db),
        }
    }
}
