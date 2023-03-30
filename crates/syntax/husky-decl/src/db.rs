use crate::*;
use husky_entity_tree::{AssociatedItem, EntityTreeResult, ImplBlock};
use husky_print_utils::p;
use husky_vfs::ModulePath;
use husky_word::Ident;
use salsa::DbWithJar;
use vec_like::VecMapGetEntry;

pub trait DeclDb: DbWithJar<DeclJar> + ExprDb {
    fn impl_block_decl(&self, impl_block: ImplBlock) -> DeclResultRef<ImplBlockDecl>;
    fn decl_sheet<'a>(&'a self, module_path: ModulePath) -> EntityTreeResult<DeclSheet<'a>>;
    fn ty_item_decl(&self, path: TypeItemPath) -> Option<TypeItemDecl>;
}

impl<Db> DeclDb for Db
where
    Db: DbWithJar<DeclJar> + ExprDb,
{
    fn decl_sheet<'a>(&'a self, module_path: ModulePath) -> EntityTreeResult<DeclSheet<'a>> {
        decl_sheet(self, module_path)
    }

    fn impl_block_decl(&self, impl_block: ImplBlock) -> DeclResultRef<ImplBlockDecl> {
        impl_block_decl(self, impl_block)
    }

    fn ty_item_decl(&self, path: TypeItemPath) -> Option<TypeItemDecl> {
        path.parent_ty(self)
            .item_decls(self)
            .ok()?
            .get_entry(path.ident(self))
            .map(|(_, decl)| decl.ok())
            .flatten()
    }
}
