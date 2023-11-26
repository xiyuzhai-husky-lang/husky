mod custom;
mod prelude;

use salsa::Database;

pub use self::custom::*;
pub use self::prelude::*;

use super::*;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id(jar = EntityPathJar)]
#[salsa::deref_id]
pub struct TypePath(ItemPathId);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TypePathData {
    module_path: ModulePath,
    ident: Ident,
    connection: MajorItemConnection,
    ty_kind: TypeKind,
}

impl TypePath {
    pub fn new(
        module_path: ModulePath,
        ident: Ident,
        connection: MajorItemConnection,
        ty_kind: TypeKind,
        db: &dyn EntityPathDb,
    ) -> Self {
        Self(ItemPathId::new(
            db,
            ItemPathData::MajorItem(MajorItemPathData::Type(TypePathData {
                module_path,
                ident,
                connection,
                ty_kind,
            })),
        ))
    }

    pub fn eqs_lifetime_ty_path(self, db: &dyn EntityPathDb) -> bool {
        self.prelude_ty_path(db) == Some(PreludeTypePath::Lifetime)
    }

    pub fn crate_path(self, db: &dyn EntityPathDb) -> CratePath {
        self.module_path(db).crate_path(db)
    }

    pub fn toolchain(self, db: &dyn EntityPathDb) -> Toolchain {
        self.crate_path(db).toolchain(db)
    }

    pub fn data(self, db: &dyn EntityPathDb) -> TypePathData {
        match self.0.data(db) {
            ItemPathData::MajorItem(MajorItemPathData::Type(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn ident(self, db: &dyn EntityPathDb) -> Ident {
        self.data(db).ident
    }

    pub fn ty_kind(self, db: &dyn EntityPathDb) -> TypeKind {
        self.data(db).ty_kind
    }

    pub fn show_aux(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityPathDb,
    ) -> std::fmt::Result {
        self.data(db).show_aux(f, db)
    }
}

impl TypePathData {
    pub(super) fn item_kind(self) -> EntityKind {
        EntityKind::MajorItem {
            module_item_kind: MajorItemKind::Type(self.ty_kind),
            connection: self.connection.kind(),
        }
    }

    pub fn module_path(&self) -> ModulePath {
        self.module_path
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn connection(&self) -> MajorItemConnection {
        self.connection
    }

    pub fn ty_kind(&self) -> TypeKind {
        self.ty_kind
    }

    #[inline(never)]
    pub fn show_aux(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityPathDb,
    ) -> std::fmt::Result {
        self.module_path.show_aux(f, db)?;
        f.write_str(show_connection(self.connection))?;
        f.write_str(self.ident.data(db))
    }
}

impl salsa::DebugWithDb for TypePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, db: &dyn ::salsa::Database) -> std::fmt::Result {
        let data = self.data(db.as_jar_db_dyn::<EntityPathJar>());
        f.write_str("TypePath(`")?;
        data.show_aux(f, db.as_jar_db_dyn::<EntityPathJar>())?;
        f.write_str("`, `")?;
        data.ty_kind.fmt(f)?;
        f.write_str("`)")
    }
}

impl salsa::DisplayWithDb for TypePath {
    fn display_with_db_fmt(&self, f: &mut std::fmt::Formatter<'_>, db: &Db) -> std::fmt::Result {
        self.show_aux(f, db.as_jar_db_dyn::<EntityPathJar>())
    }
}
