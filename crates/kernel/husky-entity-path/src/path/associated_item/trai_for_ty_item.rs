use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TraitForTypeItemPath(ItemPathId);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TraitForTypeItemPathData {
    impl_block: TraitForTypeImplBlockPath,
    ident: Ident,
    item_kind: TraitItemKind,
}

impl From<TraitForTypeItemPath> for ItemPath {
    fn from(path: TraitForTypeItemPath) -> Self {
        ItemPath::AssociatedItem(path.into())
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

impl TraitForTypeItemPathData {
    pub fn module_path(self, db: &dyn EntityPathDb) -> ModulePath {
        self.impl_block.module_path(db)
    }

    pub fn toolchain(self, db: &dyn EntityPathDb) -> Toolchain {
        self.impl_block.toolchain(db)
    }
}
