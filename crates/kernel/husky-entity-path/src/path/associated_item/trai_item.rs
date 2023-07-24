use super::*;

#[salsa::interned(db = EntityPathDb, jar = EntityPathJar)]
pub struct TraitItemPath {
    pub trai_path: TraitPath,
    pub ident: Ident,
    pub item_kind: TraitItemKind,
}

impl TraitItemPath {
    pub fn toolchain(self, db: &dyn EntityPathDb) -> Toolchain {
        self.trai_path(db).toolchain(db)
    }

    pub fn module_path(self, db: &dyn EntityPathDb) -> ModulePath {
        self.trai_path(db).module_path(db)
    }
}

impl<Db> salsa::DisplayWithDb<Db> for TraitItemPath
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

impl TraitItemPath {
    fn show_aux(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &dyn EntityPathDb,
    ) -> std::fmt::Result {
        todo!()
    }
}
