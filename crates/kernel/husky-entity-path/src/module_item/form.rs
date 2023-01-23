use super::*;
use std::fmt::Debug;

#[salsa::interned(jar = EntityPathJar, override_debug)]
pub struct FormPath {
    pub module_path: ModulePath,
    pub ident: Identifier,
    pub connection: ModuleItemConnection,
    pub form_kind: FormKind,
}

impl FormPath {
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

impl<Db: EntityPathDb + ?Sized> salsa::DebugWithDb<Db> for FormPath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<EntityPathJar>>::as_jar_db(db);
        f.write_str("FormPath(`")?;
        self.show_aux(f, db)?;
        f.write_str("`, `")?;
        self.form_kind(db).fmt(f)?;
        f.write_str("`)")
    }
}
