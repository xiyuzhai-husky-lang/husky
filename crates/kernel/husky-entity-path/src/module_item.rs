mod connection;
mod form;
mod trai;
mod ty;
mod utils;

pub use self::connection::*;
pub use self::form::*;
pub use self::trai::*;
pub use self::ty::*;

use crate::*;
use utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityPathDb)]
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
    pub fn ident(self, db: &dyn EntityPathDb) -> Ident {
        match self {
            ModuleItemPath::Type(path) => path.ident(db),
            ModuleItemPath::Trait(path) => path.ident(db),
            ModuleItemPath::Form(path) => path.ident(db),
        }
    }

    pub fn crate_path(self, db: &dyn EntityPathDb) -> CratePath {
        self.module_path(db).crate_path(db)
    }

    pub fn ty_path(self) -> Option<TypePath> {
        match self {
            ModuleItemPath::Type(path) => Some(path),
            ModuleItemPath::Trait(_) | ModuleItemPath::Form(_) => None,
        }
    }

    pub(crate) fn entity_kind(self, db: &dyn EntityPathDb) -> EntityKind {
        match self {
            ModuleItemPath::Type(path) => EntityKind::ModuleItem {
                module_item_kind: ModuleItemKind::Type(path.ty_kind(db)),
                connection: path.connection(db).kind(),
            },
            ModuleItemPath::Trait(path) => EntityKind::ModuleItem {
                module_item_kind: ModuleItemKind::Trait,
                connection: path.connection(db).kind(),
            },
            ModuleItemPath::Form(path) => EntityKind::ModuleItem {
                module_item_kind: ModuleItemKind::Form(path.form_kind(db)),
                connection: path.connection(db).kind(),
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

impl<Db> salsa::DisplayWithDb<Db> for ModuleItemPath
where
    Db: EntityPathDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityPathJar>>::as_jar_db(db);
        if level.is_root() {
            match self {
                ModuleItemPath::Form(path) => path.display_with_db_fmt(f, db, level.next()),
                ModuleItemPath::Type(path) => path.display_with_db_fmt(f, db, level.next()),
                ModuleItemPath::Trait(path) => path.display_with_db_fmt(f, db, level.next()),
            }
        } else {
            f.write_str(self.ident(db).data(db))
        }
    }
}
