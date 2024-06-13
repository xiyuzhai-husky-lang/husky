use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::as_id]
#[salsa::deref_id]
pub struct TraitItemPath(ItemPathId);

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TraitItemPathData {
    pub(crate) trai_path: TraitPath,
    pub(crate) ident: Ident,
    pub(crate) item_kind: TraitItemKind,
}

impl TraitItemPath {
    pub fn new(
        trai_path: TraitPath,
        ident: Ident,
        item_kind: TraitItemKind,
        db: &::salsa::Db,
    ) -> Self {
        Self(ItemPathId::new(
            db,
            ItemPathData::AssocItem(AssocItemPathData::TraitItem(TraitItemPathData {
                trai_path,
                ident,
                item_kind,
            })),
        ))
    }

    pub fn data(self, db: &::salsa::Db) -> TraitItemPathData {
        match self.0.data(db) {
            ItemPathData::AssocItem(AssocItemPathData::TraitItem(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn trai_path(self, db: &::salsa::Db) -> TraitPath {
        self.data(db).trai_path
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.data(db).ident
    }

    pub fn item_kind(self, db: &::salsa::Db) -> TraitItemKind {
        self.data(db).item_kind
    }
}

impl TraitItemPathData {
    #[inline(always)]
    pub(super) fn item_path(self, id: ItemPathId) -> TraitItemPath {
        TraitItemPath(id)
    }

    pub fn trai_path(&self) -> TraitPath {
        self.trai_path
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn item_kind(&self) -> TraitItemKind {
        self.item_kind
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.trai_path.module_path(db)
    }

    pub fn entity_kind(self, _db: &::salsa::Db) -> EntityKind {
        EntityKind::AssocItem {
            assoc_item_kind: AssocItemKind::TraitItem(self.item_kind),
        }
    }

    #[inline(never)]
    fn show_aux(self, f: &mut std::fmt::Formatter<'_>, db: &::salsa::Db) -> std::fmt::Result {
        self.trai_path.show_aux(f, db)?;
        f.write_str("::")?;
        f.write_str(self.ident.data(db))
    }
}

impl salsa::DebugWithDb for TraitItemPath {
    fn debug_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        item_debug_fmt_with_db!(self, f, "TraitItemPath", db)
    }
}

impl salsa::DisplayWithDb for TraitItemPath {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.data(db).show_aux(f, db)
    }
}
