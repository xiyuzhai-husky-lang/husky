use super::*;
use std::fmt::Debug;

#[salsa::interned(jar = EntityPathJar, override_debug)]
pub struct FugitivePath {
    pub module_path: ModulePath,
    pub ident: Ident,
    pub connection: MajorItemConnection,
    pub fugitive_kind: FugitiveKind,
}

impl FugitivePath {
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

impl<Db: EntityPathDb + ?Sized> salsa::DebugWithDb<Db> for FugitivePath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<EntityPathJar>>::as_jar_db(db);
        f.write_str("FugitivePath(`")?;
        self.show_aux(f, db)?;
        f.write_str("`, `")?;
        self.fugitive_kind(db).fmt(f)?;
        f.write_str("`)")
    }
}

impl<Db> salsa::DisplayWithDb<Db> for FugitivePath
where
    Db: EntityPathDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityPathJar>>::as_jar_db(db);
        self.show_aux(f, db)
    }
}
