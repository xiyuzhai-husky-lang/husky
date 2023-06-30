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
}

impl<Db> salsa::DisplayWithDb<Db> for TypeVariantPath
where
    Db: EntityPathDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &Db,
        _level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        todo!()
    }
}
