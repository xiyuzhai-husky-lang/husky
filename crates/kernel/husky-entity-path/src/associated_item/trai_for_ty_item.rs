use super::*;

#[salsa::interned(db = EntityPathDb, jar = EntityPathJar)]
pub struct TraitForTypeItemPath {
    pub parent_ty: TypePath,
    pub trai: TraitPath,
    pub ident: Ident,
    pub trai_for_ty_item_kind: TraitItemKind,
}

impl<Db> salsa::DisplayWithDb<Db> for TraitForTypeItemPath
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

impl TraitForTypeItemPath {
    fn show_aux(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &dyn EntityPathDb,
    ) -> std::fmt::Result {
        todo!()
    }
}
