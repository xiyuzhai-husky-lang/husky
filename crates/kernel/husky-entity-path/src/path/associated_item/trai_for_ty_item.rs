use super::*;

#[salsa::interned(db = EntityPathDb, jar = EntityPathJar)]
pub struct TraitForTypeItemPath {
    pub module_path: ModulePath,
    pub ty_path: TypePath,
    pub trai_path: TraitPath,
    pub ident: Ident,
    pub item_kind: TraitItemKind,
}

impl From<TraitForTypeItemPath> for EntityPath {
    fn from(path: TraitForTypeItemPath) -> Self {
        EntityPath::AssociatedItem(path.into())
    }
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
