use super::*;

#[salsa::interned(db = EntityPathDb, jar = EntityPathJar)]
pub struct TypeItemPath {
    pub ty_path: TypePath,
    pub ident: Ident,
    pub item_kind: TypeItemKind,
}

impl TypeItemPath {
    pub fn toolchain(self, db: &dyn EntityPathDb) -> Toolchain {
        self.ty_path(db).toolchain(db)
    }

    pub fn module_path(self, db: &dyn EntityPathDb) -> ModulePath {
        self.ty_path(db).module_path(db)
    }

    fn show_aux(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &dyn EntityPathDb,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl<Db> salsa::DisplayWithDb<Db> for TypeItemPath
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
