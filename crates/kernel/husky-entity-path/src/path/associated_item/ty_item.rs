use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TypeItemPath(ItemPathId);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TypeItemPathData {
    impl_block: TypeImplBlockPath,
    ident: Ident,
    item_kind: TypeItemKind,
}

impl TypeItemPath {
    pub fn data(self, db: &dyn EntityPathDb) -> TypeItemPathData {
        match self.0.data(db) {
            ItemPathData::AssociatedItem(AssociatedItemPathData::TypeItem(data)) => data,
            _ => unreachable!(),
        }
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
