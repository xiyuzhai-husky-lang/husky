mod form;
mod trai;
mod ty;
mod utils;

pub use form::*;
pub use trai::*;
pub use ty::*;

use crate::*;
use utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleItemPath {
    Type(TypePath),
    Trait(TraitPath),
    Form(FormPath),
}

impl ModuleItemPath {
    pub fn module_path(self, db: &dyn EntityPathDb) -> ModulePath {
        match self {
            ModuleItemPath::Type(path) => path.module_path(db),
            ModuleItemPath::Trait(path) => path.module_path(db),
            ModuleItemPath::Form(path) => path.module_path(db),
        }
    }
    pub fn ident(self, db: &dyn EntityPathDb) -> Identifier {
        match self {
            ModuleItemPath::Type(path) => path.ident(db),
            ModuleItemPath::Trait(path) => path.ident(db),
            ModuleItemPath::Form(path) => path.ident(db),
        }
    }

    pub fn crate_path(self, db: &dyn EntityPathDb) -> CratePath {
        self.module_path(db).crate_path(db)
    }

    pub(crate) fn entity_kind(self, db: &dyn EntityPathDb) -> EntityKind {
        match self {
            ModuleItemPath::Type(path) => EntityKind::ModuleItem {
                module_item_kind: ModuleItemKind::Type(path.ty_kind(db)),
                connection: path.connection(db),
            },
            ModuleItemPath::Trait(path) => EntityKind::ModuleItem {
                module_item_kind: ModuleItemKind::Trait,
                connection: path.connection(db),
            },
            ModuleItemPath::Form(path) => EntityKind::ModuleItem {
                module_item_kind: ModuleItemKind::Form(path.form_kind(db)),
                connection: path.connection(db),
            },
        }
    }
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
