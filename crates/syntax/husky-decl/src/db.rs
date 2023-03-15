use crate::*;
use husky_entity_tree::{AssociatedItem, EntityTreeResult, ImplBlock};
use husky_print_utils::p;
use husky_vfs::ModulePath;
use husky_word::Ident;
use salsa::DbWithJar;
use vec_like::VecMapGetEntry;

pub trait DeclDb: DbWithJar<DeclJar> + ExprDb {
    fn module_item_decl(&self, module_path: ModuleItemPath) -> DeclResultRef<Decl>;
    fn ty_decl(&self, path: TypePath) -> DeclResultRef<TypeDecl>;
    fn trai_decl(&self, path: TraitPath) -> DeclResultRef<TraitDecl>;
    fn form_decl(&self, path: FormPath) -> DeclResultRef<FormDecl>;
    fn impl_decl(&self, impl_block: ImplBlock) -> DeclResultRef<ImplDecl>;
    fn associated_item_decl(
        &self,
        associated_item: AssociatedItem,
    ) -> DeclResultRef<AssociatedItemDecl>;
    fn decl_sheet<'a>(&'a self, module_path: ModulePath) -> EntityTreeResult<DeclSheet<'a>>;
    fn ty_item_decls(
        &self,
        path: TypePath,
    ) -> EntityTreeBundleResultRef<&[(Ident, Result<TypeItemDecl, ()>)]>;
    fn ty_item_decl(&self, path: TypeItemPath) -> Option<TypeItemDecl>;
}

impl<Db> DeclDb for Db
where
    Db: DbWithJar<DeclJar> + ExprDb,
{
    fn module_item_decl(&self, module_path: ModuleItemPath) -> DeclResultRef<Decl> {
        module_item_decl(self, module_path)
    }

    fn ty_decl(&self, path: TypePath) -> DeclResultRef<TypeDecl> {
        ty_decl(self, path).as_ref().copied()
    }

    fn trai_decl(&self, path: TraitPath) -> DeclResultRef<TraitDecl> {
        trai_decl(self, path).as_ref().copied()
    }

    fn form_decl(&self, path: FormPath) -> DeclResultRef<FormDecl> {
        form_decl(self, path).as_ref().copied()
    }

    fn decl_sheet<'a>(&'a self, module_path: ModulePath) -> EntityTreeResult<DeclSheet<'a>> {
        decl_sheet(self, module_path)
    }

    fn impl_decl(&self, impl_block: ImplBlock) -> DeclResultRef<ImplDecl> {
        impl_decl(self, impl_block)
    }

    fn associated_item_decl(
        &self,
        associated_item: AssociatedItem,
    ) -> DeclResultRef<AssociatedItemDecl> {
        associated_item_decl(self, associated_item)
            .as_ref()
            .copied()
    }

    fn ty_item_decls(
        &self,
        path: TypePath,
    ) -> EntityTreeBundleResultRef<&[(Ident, Result<TypeItemDecl, ()>)]> {
        match ty_item_decls(self, path) {
            Ok(ty_item_decls) => Ok(ty_item_decls),
            Err(e) => Err(e),
        }
    }

    fn ty_item_decl(&self, path: TypeItemPath) -> Option<TypeItemDecl> {
        self.ty_item_decls(path.ty(self))
            .ok()?
            .get_entry(path.ident(self))
            .map(|(_, decl)| decl.ok())
            .flatten()
    }
}
