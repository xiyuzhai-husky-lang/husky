use crate::*;

use husky_ast::AstDb;

use husky_entity_kind::EntityKind;
use husky_entity_path::EntityPath;
use husky_manifest::ManifestDb;
use husky_vfs::*;

use salsa::DbWithJar;

pub trait EntityTreeDb: DbWithJar<EntityTreeJar> + AstDb + EntityPathDb + ManifestDb {
    fn module_item_entity_kind(
        &self,
        module_item_path: ModuleItemPath,
    ) -> &EntityTreeResult<EntityKind>;
    fn associated_item_entity_kind(
        &self,
        associated_item_path: AssociatedItemPath,
    ) -> &EntityTreeResult<EntityKind>;
    fn submodules(&self, module_path: ModulePath) -> VfsResult<&[ModulePath]>;
    fn all_modules_within_crate(&self, crate_path: CratePath) -> VfsResult<&[ModulePath]>;
}

impl<T> EntityTreeDb for T
where
    T: DbWithJar<EntityTreeJar> + AstDb + EntityPathDb + ManifestDb,
{
    fn module_item_entity_kind(
        &self,
        module_item_path: ModuleItemPath,
    ) -> &EntityTreeResult<EntityKind> {
        todo!()
    }
    fn associated_item_entity_kind(
        &self,
        associated_item_path: AssociatedItemPath,
    ) -> &EntityTreeResult<EntityKind> {
        todo!()
    }

    fn submodules(&self, module_path: ModulePath) -> VfsResult<&[ModulePath]> {
        Ok(submodules(self, module_path).as_ref()?)
    }

    fn all_modules_within_crate(&self, crate_path: CratePath) -> VfsResult<&[ModulePath]> {
        Ok(all_modules_within_crate(self, crate_path).as_ref()?)
    }
}
