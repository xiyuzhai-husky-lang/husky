use crate::*;

#[salsa::interned(db = EntityPathDb, jar = EntityPathJar)]
pub struct TypeVariantPath {
    pub parent_ty_path: TypePath,
    pub ident: Ident,
}

impl TypeVariantPath {
    pub fn toolchain(self, db: &dyn EntityPathDb) -> Toolchain {
        self.parent_ty_path(db).toolchain(db)
    }

    pub fn module_path(self, db: &dyn EntityPathDb) -> ModulePath {
        self.parent_ty_path(db).module_path(db)
    }

    pub fn show_aux(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityPathDb,
    ) -> std::fmt::Result {
        self.parent_ty_path(db).show_aux(f, db)?;
        f.write_str("::")?;
        f.write_str(self.ident(db).data(db))
    }
}

impl<Db> salsa::DisplayWithDb<Db> for TypeVariantPath
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
