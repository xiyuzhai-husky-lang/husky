use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::deref_id]
pub struct TypeItemPath(ItemPathId);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TypeItemPathData {
    impl_block: TypeImplBlockPath,
    ident: Ident,
    item_kind: TypeItemKind,
}

impl TypeItemPath {
    pub fn new(
        impl_block: TypeImplBlockPath,
        ident: Ident,
        item_kind: TypeItemKind,
        db: &::salsa::Db,
    ) -> Self {
        Self(ItemPathId::new(
            db,
            ItemPathData::AssociatedItem(AssociatedItemPathData::TypeItem(TypeItemPathData {
                impl_block,
                ident,
                item_kind,
            })),
        ))
    }

    pub fn data(self, db: &::salsa::Db) -> TypeItemPathData {
        match self.0.data(db) {
            ItemPathData::AssociatedItem(AssociatedItemPathData::TypeItem(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn impl_block(self, db: &::salsa::Db) -> TypeImplBlockPath {
        self.data(db).impl_block
    }

    pub fn item_kind(self, db: &::salsa::Db) -> TypeItemKind {
        self.data(db).item_kind
    }

    #[inline(never)]
    fn show_aux(self, _f: &mut std::fmt::Formatter<'_>, _db: &::salsa::Db) -> std::fmt::Result {
        todo!()
    }
}

impl salsa::DisplayWithDb for TypeItemPath {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_aux(f, db)
    }
}

impl TypeItemPathData {
    pub fn impl_block(self) -> TypeImplBlockPath {
        self.impl_block
    }

    pub fn ident(self) -> Ident {
        self.ident
    }

    pub fn item_kind(self) -> TypeItemKind {
        self.item_kind
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.impl_block.module_path(db)
    }
}
