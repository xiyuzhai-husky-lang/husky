use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::deref_id]
pub struct TraitItemPath(ItemPathId);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TraitItemPathData {
    trai_path: TraitPath,
    ident: Ident,
    item_kind: TraitItemKind,
}

impl TraitItemPath {
    pub fn new(
        trai_path: TraitPath,
        ident: Ident,
        item_kind: TraitItemKind,
        db: &dyn EntityPathDb,
    ) -> Self {
        Self(ItemPathId::new(
            db,
            ItemPathData::AssociatedItem(AssociatedItemPathData::TraitItem(TraitItemPathData {
                trai_path,
                ident,
                item_kind,
            })),
        ))
    }

    pub fn data(self, db: &dyn EntityPathDb) -> TraitItemPathData {
        match self.0.data(db) {
            ItemPathData::AssociatedItem(AssociatedItemPathData::TraitItem(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn trai_path(self, db: &dyn EntityPathDb) -> TraitPath {
        self.data(db).trai_path
    }

    pub fn item_kind(self, db: &dyn EntityPathDb) -> TraitItemKind {
        self.data(db).item_kind
    }

    #[inline(never)]
    fn show_aux(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &dyn EntityPathDb,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl salsa::DisplayWithDb for TraitItemPath {
    fn display_with_db_fmt(&self, f: &mut std::fmt::Formatter<'_>, db: &Db) -> std::fmt::Result {
        self.show_aux(f, db.as_jar_db_dyn::<EntityPathJar>())
    }
}

impl TraitItemPathData {
    pub fn trai_path(&self) -> TraitPath {
        self.trai_path
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn item_kind(&self) -> TraitItemKind {
        self.item_kind
    }

    pub fn module_path(self, db: &dyn EntityPathDb) -> ModulePath {
        self.trai_path.module_path(db)
    }
}
