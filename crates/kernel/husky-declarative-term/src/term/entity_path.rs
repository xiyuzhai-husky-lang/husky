use husky_entity_path::*;
use salsa::DisplayWithDb;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
#[enum_class::from_variants]
pub enum DeclarativeTermEntityPath {
    Form(FugitivePath),
    Trait(TraitPath),
    Type(TypePath),
}

impl DeclarativeTermEntityPath {
    pub fn ty_path(self) -> Option<TypePath> {
        match self {
            DeclarativeTermEntityPath::Type(path) => Some(path),
            DeclarativeTermEntityPath::Form(_) | DeclarativeTermEntityPath::Trait(_) => None,
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

impl DeclarativeTermEntityPath {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn DeclarativeTermDb,
        _ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        match self {
            DeclarativeTermEntityPath::Form(path) => {
                path.display_with_db_fmt(f, db, salsa::DisplayFormatLevel::root())
            }
            DeclarativeTermEntityPath::Trait(path) => {
                path.display_with_db_fmt(f, db, salsa::DisplayFormatLevel::root())
            }
            DeclarativeTermEntityPath::Type(path) => {
                path.display_with_db_fmt(f, db, salsa::DisplayFormatLevel::root())
            }
        }
    }
}
