mod form;
mod trai;
mod ty;

pub use form::*;
pub use trai::*;
pub use ty::*;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleItemPath {
    Type(TypePath),
    Trait(TraitPath),
    Form(FormPath),
}

impl From<FormPath> for ModuleItemPath {
    fn from(v: FormPath) -> Self {
        Self::Form(v)
    }
}

impl From<TraitPath> for ModuleItemPath {
    fn from(v: TraitPath) -> Self {
        Self::Trait(v)
    }
}

impl From<TypePath> for ModuleItemPath {
    fn from(v: TypePath) -> Self {
        Self::Type(v)
    }
}

impl<Db> salsa::DebugWithDb<Db> for ModuleItemPath
where
    Db: EntityPathDb + ?Sized,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<EntityPathJar>>::as_jar_db(db);
        if include_all_fields {
            match self {
                ModuleItemPath::Form(path) => f
                    .debug_tuple("Form")
                    .field(&path.debug_with(db, include_all_fields))
                    .finish(),
                ModuleItemPath::Trait(path) => f
                    .debug_tuple("Trait")
                    .field(&path.debug_with(db, include_all_fields))
                    .finish(),
                ModuleItemPath::Type(path) => f
                    .debug_tuple("Type")
                    .field(&path.debug_with(db, include_all_fields))
                    .finish(),
            }
        } else {
            match self {
                ModuleItemPath::Form(path) => path.fmt(f, db, false),
                ModuleItemPath::Type(path) => path.fmt(f, db, false),
                ModuleItemPath::Trait(path) => path.fmt(f, db, false),
            }
        }
    }
}
