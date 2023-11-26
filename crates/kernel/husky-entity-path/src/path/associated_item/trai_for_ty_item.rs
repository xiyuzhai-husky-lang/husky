use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::deref_id]
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

impl TraitForTypeItemPath {
    pub fn new(
        impl_block: TraitForTypeImplBlockPath,
        ident: Ident,
        item_kind: TraitItemKind,
        db: &dyn EntityPathDb,
    ) -> Self {
        Self(ItemPathId::new(
            db,
            ItemPathData::AssociatedItem(AssociatedItemPathData::TraitForTypeItem(
                TraitForTypeItemPathData {
                    impl_block,
                    ident,
                    item_kind,
                },
            )),
        ))
    }

    pub fn data(self, db: &dyn EntityPathDb) -> TraitForTypeItemPathData {
        match self.0.data(db) {
            ItemPathData::AssociatedItem(AssociatedItemPathData::TraitForTypeItem(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn impl_block(self, db: &dyn EntityPathDb) -> TraitForTypeImplBlockPath {
        self.data(db).impl_block
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

impl TraitForTypeItemPathData {
    pub fn module_path(self, db: &dyn EntityPathDb) -> ModulePath {
        self.impl_block.module_path(db)
    }

    pub fn toolchain(self, db: &dyn EntityPathDb) -> Toolchain {
        self.impl_block.toolchain(db)
    }
}

impl salsa::DisplayWithDb for TraitForTypeItemPath {
    fn display_with_db_fmt(&self, f: &mut std::fmt::Formatter<'_>, db: &Db) -> std::fmt::Result {
        self.show_aux(f, db())
    }
}
