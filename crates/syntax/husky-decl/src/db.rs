use crate::*;
use husky_entity_tree::{AssociatedItem, EntityTreeDb, EntityTreeResult, ImplBlock};
use husky_vfs::{CratePath, ModulePath, VfsResult};
use salsa::DbWithJar;

pub trait DeclDb: DbWithJar<DeclJar> + ExprDb {
    fn module_item_decl(&self, module_path: ModuleItemPath) -> DeclResultBorrowed<Decl>;
    fn impl_block_decl(&self, impl_block: ImplBlock) -> DeclResultBorrowed<ImplBlockDecl>;
    fn associated_item_decl(
        &self,
        associated_item: AssociatedItem,
    ) -> DeclResultBorrowed<AssociatedItemDecl>;
    fn module_decl_sheet<'a>(&'a self, module_path: ModulePath) -> EntityTreeResult<DeclSheet<'a>>;
}

impl<Db> DeclDb for Db
where
    Db: DbWithJar<DeclJar> + ExprDb,
{
    fn module_item_decl(&self, module_path: ModuleItemPath) -> DeclResultBorrowed<Decl> {
        module_item_decl(self, module_path)
    }
    fn module_decl_sheet<'a>(&'a self, module_path: ModulePath) -> EntityTreeResult<DeclSheet<'a>> {
        module_decl_sheet(self, module_path)
    }

    fn impl_block_decl(&self, impl_block: ImplBlock) -> DeclResultBorrowed<ImplBlockDecl> {
        impl_block_decl(self, impl_block).as_ref().map(|decl| *decl)
    }

    fn associated_item_decl(
        &self,
        associated_item: AssociatedItem,
    ) -> DeclResultBorrowed<AssociatedItemDecl> {
        associated_item_decl(self, associated_item)
            .as_ref()
            .map(|decl| *decl)
    }
}
