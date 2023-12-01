use std::fmt::Debug;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::as_id]
#[salsa::deref_id]
pub struct TypeItemPath(ItemPathId);

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TypeItemPathData {
    pub(crate) impl_block: TypeImplBlockPath,
    pub(crate) ident: Ident,
    pub(crate) item_kind: TypeItemKind,
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
}

impl TypeItemPathData {
    fn show_aux(self, f: &mut std::fmt::Formatter<'_>, db: &::salsa::Db) -> std::fmt::Result {
        f.write_str("(")?;
        self.impl_block.show_aux(f, db)?;
        f.write_str("::");
        f.write_str(self.ident.data(db))
    }
}

impl salsa::DebugWithDb for TypeItemPath {
    fn debug_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        let data = self.data(db);
        f.write_str("TypeItemPath(`")?;
        data.show_aux(f, db)?;
        f.write_str("`, `")?;
        data.item_kind.fmt(f)?;
        f.write_str("`)")
    }
}

impl salsa::DisplayWithDb for TypeItemPath {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl TypeItemPathData {
    #[inline(always)]
    pub(super) fn item_path(self, id: ItemPathId) -> TypeItemPath {
        TypeItemPath(id)
    }

    pub fn impl_block(self) -> TypeImplBlockPath {
        self.impl_block
    }

    pub fn ident(self) -> Ident {
        self.ident
    }

    pub fn item_kind(self) -> TypeItemKind {
        self.item_kind
    }

    pub fn entity_kind(self, db: &::salsa::Db) -> EntityKind {
        EntityKind::AssociatedItem {
            associated_item_kind: AssociatedItemKind::TypeItem(self.item_kind),
        }
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.impl_block.module_path(db)
    }
}
