use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::as_id(jar = EntityPathJar)]
#[salsa::deref_id]
pub struct TypeVariantPath(ItemPathId);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TypeVariantPathData {
    parent_ty_path: TypePath,
    ident: Ident,
}

impl TypeVariantPath {
    pub fn new(parent_ty_path: TypePath, ident: Ident, db: &dyn EntityPathDb) -> Self {
        Self(ItemPathId::new(
            db,
            ItemPathData::TypeVariant(TypeVariantPathData {
                parent_ty_path,
                ident,
            }),
        ))
    }

    pub fn data(self, db: &dyn EntityPathDb) -> TypeVariantPathData {
        match self.0.data(db) {
            ItemPathData::TypeVariant(data) => data,
            _ => unreachable!(),
        }
    }

    pub fn parent_ty_path(self, db: &dyn EntityPathDb) -> TypePath {
        self.data(db).parent_ty_path
    }

    pub fn ident(self, db: &dyn EntityPathDb) -> Ident {
        self.data(db).ident
    }

    #[inline(never)]
    pub fn show_aux(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityPathDb,
    ) -> std::fmt::Result {
        self.data(db).show_aux(f, db)
    }
}

impl TypeVariantPathData {
    pub fn toolchain(self, db: &dyn EntityPathDb) -> Toolchain {
        self.parent_ty_path.toolchain(db)
    }

    pub fn module_path(self, db: &dyn EntityPathDb) -> ModulePath {
        self.parent_ty_path.module_path(db)
    }

    #[inline(never)]
    pub fn show_aux(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityPathDb,
    ) -> std::fmt::Result {
        self.parent_ty_path.show_aux(f, db)?;
        f.write_str("::")?;
        f.write_str(self.ident.data(db))
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
