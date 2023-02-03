use super::*;

#[salsa::interned(db = EntityPathDb, jar = EntityPathJar)]
pub struct TraitItemPath {
    pub trai_path: TraitPath,
    pub ident: Identifier,
}

impl<Db> salsa::DisplayWithDb<Db> for TraitItemPath
where
    Db: EntityPathDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityPathJar>>::as_jar_db(db);
        self.show_aux(f, db)
    }
}

impl TraitItemPath {
    fn show_aux(self, f: &mut std::fmt::Formatter<'_>, db: &dyn EntityPathDb) -> std::fmt::Result {
        todo!()
    }
}
