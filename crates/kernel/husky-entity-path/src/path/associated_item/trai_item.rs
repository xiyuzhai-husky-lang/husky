use super::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::as_id]
#[salsa::deref_id]
pub struct TraitItemPath(ItemPathId);

#[salsa::debug_with_db]
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
            ItemPathData::AssociatedItem(AssociatedItemPathData::TraitItem(TraitItemPathData {
                trai_path,
                ident,
                item_kind,
            })),
        ))
    }

    pub fn data(self, db: &::salsa::Db) -> TraitItemPathData {
        match self.0.data(db) {
            ItemPathData::AssociatedItem(AssociatedItemPathData::TraitItem(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn trai_path(self, db: &::salsa::Db) -> TraitPath {
        self.data(db).trai_path
    }

    pub fn item_kind(self, db: &::salsa::Db) -> TraitItemKind {
        self.data(db).item_kind
    }

    #[inline(never)]
    fn show_aux(self, _f: &mut std::fmt::Formatter<'_>, _db: &::salsa::Db) -> std::fmt::Result {
        todo!()
    }
}

impl salsa::DisplayWithDb for TraitItemPath {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_aux(f, db)
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

    pub fn entity_kind(self, db: &::salsa::Db) -> EntityKind {
        EntityKind::AssociatedItem {
            associated_item_kind: AssociatedItemKind::TraitItem(self.item_kind),
        }
    }
}
