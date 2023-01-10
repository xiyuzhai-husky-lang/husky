use crate::*;
use husky_entity_tree::{AssociatedItem, EntityTreeDb, EntityTreeResult, ImplBlock};
use husky_vfs::{CratePath, ModulePath, VfsResult};
use salsa::DbWithJar;

pub trait DeclDb: DbWithJar<DeclJar> + ExprDb {
    fn module_item_decl(&self, path: ModuleItemPath) -> DeclResult<Decl>;
    fn impl_block_decl(&self, impl_block: ImplBlock) -> DeclResult<ImplBlockDecl>;
    fn associated_item_decl(
        &self,
        associated_item: AssociatedItem,
    ) -> DeclResult<AssociatedItemDecl>;
    fn module_decl_sheet(&self, path: ModulePath) -> EntityTreeResult<&DeclSheet>;
}

impl<Db> DeclDb for Db
where
    Db: DbWithJar<DeclJar> + ExprDb,
{
    fn module_item_decl(&self, path: ModuleItemPath) -> DeclResult<Decl> {
        module_item_decl(self, path)
    }
    fn module_decl_sheet(&self, path: ModulePath) -> EntityTreeResult<&DeclSheet> {
        Ok(module_decl_sheet(self, path)
            .as_ref()
            .map_err(|e| e.clone())?)
    }

    fn impl_block_decl(&self, impl_block: ImplBlock) -> DeclResult<ImplBlockDecl> {
        impl_block_decl(self, impl_block)
    }

    fn associated_item_decl(
        &self,
        associated_item: AssociatedItem,
    ) -> DeclResult<AssociatedItemDecl> {
        associated_item_decl(self, associated_item)
    }
}
