mod connection;
mod fugitive;
mod trai;
mod ty;
mod utils;

pub use self::connection::*;
pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use crate::*;
use utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntityPathDb)]
#[enum_class::from_variants]
pub enum MajorItemPath {
    Type(TypePath),
    Trait(TraitPath),
    Fugitive(FugitivePath),
}

impl MajorItemPath {
    pub fn module_path(self, db: &dyn EntityPathDb) -> ModulePath {
        match self {
            MajorItemPath::Type(path) => path.module_path(db),
            MajorItemPath::Trait(path) => path.module_path(db),
            MajorItemPath::Fugitive(path) => path.module_path(db),
        }
    }
    pub fn ident(self, db: &dyn EntityPathDb) -> Ident {
        match self {
            MajorItemPath::Type(path) => path.ident(db),
            MajorItemPath::Trait(path) => path.ident(db),
            MajorItemPath::Fugitive(path) => path.ident(db),
        }
    }

    pub fn crate_path(self, db: &dyn EntityPathDb) -> CratePath {
        self.module_path(db).crate_path(db)
    }

    pub fn ty_path(self) -> Option<TypePath> {
        match self {
            MajorItemPath::Type(path) => Some(path),
            MajorItemPath::Trait(_) | MajorItemPath::Fugitive(_) => None,
        }
    }

    pub(crate) fn item_kind(self, db: &dyn EntityPathDb) -> EntityKind {
        match self {
            MajorItemPath::Type(path) => EntityKind::MajorItem {
                module_item_kind: MajorItemKind::Type(path.ty_kind(db)),
                connection: path.connection(db).kind(),
            },
            MajorItemPath::Trait(path) => EntityKind::MajorItem {
                module_item_kind: MajorItemKind::Trait,
                connection: path.connection(db).kind(),
            },
            MajorItemPath::Fugitive(path) => EntityKind::MajorItem {
                module_item_kind: MajorItemKind::Fugitive(path.fugitive_kind(db)),
                connection: path.connection(db).kind(),
            },
        }
    }
}

impl<Db> salsa::DisplayWithDb<Db> for MajorItemPath
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
                MajorItemPath::Fugitive(path) => path.display_with_db_fmt(f, db, level.next()),
                MajorItemPath::Type(path) => path.display_with_db_fmt(f, db, level.next()),
                MajorItemPath::Trait(path) => path.display_with_db_fmt(f, db, level.next()),
            }
        } else {
            f.write_str(self.ident(db).data(db))
        }
    }
}
