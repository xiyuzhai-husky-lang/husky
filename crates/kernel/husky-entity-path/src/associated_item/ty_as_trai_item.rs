use super::*;

#[salsa::interned(db = EntityPathDb, jar = EntityPathJar)]
pub struct TypeAsTraitItemPath {
    pub parent_ty: TypePath,
    pub trai: TraitPath,
    pub ident: Ident,
    pub ty_as_trai_item_kind: TraitItemKind,
}

impl<Db> salsa::DisplayWithDb<Db> for TypeAsTraitItemPath
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

impl TypeAsTraitItemPath {
    fn show_aux(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &dyn EntityPathDb,
    ) -> std::fmt::Result {
        todo!()
    }
}
