use super::*;
use std::fmt::Debug;

#[salsa::interned(jar = EntityPathJar, override_debug)]
pub struct TypePath {
    pub module_path: ModulePath,
    pub ident: Identifier,
    pub connection: ModuleItemConnection,
    pub ty_kind: TypeKind,
}

impl TypePath {
    pub fn eqs_lifetime_ty_path(self, db: &dyn EntityPathDb) -> bool {
        is_ty_path_lifetime_ty(db, self)
    }

    pub fn show_aux(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityPathDb,
    ) -> std::fmt::Result {
        self.module_path(db).show_aux(f, db)?;
        f.write_str(show_connection(self.connection(db)))?;
        f.write_str(self.ident(db).data(db))
    }

    pub fn crate_path(self, db: &dyn EntityPathDb) -> CratePath {
        self.module_path(db).crate_path(db)
    }

    pub fn toolchain(self, db: &dyn EntityPathDb) -> Toolchain {
        self.crate_path(db).toolchain(db)
    }
}

#[salsa::tracked(jar = EntityPathJar)]
pub(crate) fn is_ty_path_lifetime_ty(db: &dyn EntityPathDb, ty_path: TypePath) -> bool {
    let toolchain = ty_path.toolchain(db);
    let Ok(entity_path_menu) = db.entity_path_menu(toolchain) else {
        return false
    };
    ty_path == entity_path_menu.lifetime_ty_path()
}

impl<Db: EntityPathDb + ?Sized> salsa::DebugWithDb<Db> for TypePath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<EntityPathJar>>::as_jar_db(db);
        f.write_str("TypePath(`")?;
        self.show_aux(f, db)?;
        f.write_str("`, `")?;
        self.ty_kind(db).fmt(f)?;
        f.write_str("`)")
    }
}

impl<Db> salsa::DisplayWithDb<Db> for TypePath
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
        self.show_aux(f, db)
    }
}
