use crate::*;

use husky_ast::AstDb;

use husky_entity_path::EntityPath;
use husky_entity_taxonomy::EntityKind;
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
    fn all_modules_within_crate(&self, crate_path: CratePath) -> &[ModulePath];
    fn entity_tree_bundle(
        &self,
        crate_path: CratePath,
    ) -> EntityTreeBundleResult<&EntityTreeBundle>;
    fn entity_tree_sheet(&self, module_path: ModulePath) -> EntityTreeResult<&EntityTreeSheet>;
    fn module_prelude<'a>(&'a self, module_path: ModulePath) -> PreludeResult<ModulePrelude<'a>>;
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

    fn all_modules_within_crate(&self, crate_path: CratePath) -> &[ModulePath] {
        all_modules_within_crate(self, crate_path)
    }

    fn entity_tree_bundle(
        &self,
        crate_path: CratePath,
    ) -> EntityTreeBundleResult<&EntityTreeBundle> {
        Ok(entity_tree_bundle(self, crate_path)
            .as_ref()
            .map_err(|e| e.clone())?)
    }

    fn entity_tree_sheet(&self, module_path: ModulePath) -> EntityTreeResult<&EntityTreeSheet> {
        entity_tree_sheet(self, module_path)
    }

    fn module_prelude<'a>(&'a self, module_path: ModulePath) -> PreludeResult<ModulePrelude<'a>> {
        module_prelude(self, module_path)
    }
}
