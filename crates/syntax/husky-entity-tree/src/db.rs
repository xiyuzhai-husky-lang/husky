use crate::*;

use husky_ast::AstDb;

use husky_entity_path::EntityPath;
use husky_entity_taxonomy::EntityKind;
use husky_manifest::ManifestDb;
use husky_vfs::*;

use husky_word::IdentPairMap;
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
    fn entity_tree_crate_bundle(
        &self,
        crate_path: CratePath,
    ) -> EntityTreeCrateBundleResult<&EntityTreeCrateBundle>;
    fn entree_module_sheet(
        &self,
        module_path: ModulePath,
    ) -> EntityTreeResult<&EntityTreeModuleSheet>;
    fn module_symbol_context<'a>(
        &'a self,
        module_path: ModulePath,
    ) -> EntityTreeResult<ModuleSymbolContext<'a>>;
    fn subentity_path(
        &self,
        parent: EntityPath,
        identifier: Identifier,
    ) -> EntityTreeResult<EntityPath>;
    fn impl_block_associated_items(&self, impl_block: ImplBlock) -> &IdentPairMap<AssociatedItem>;
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

    fn entity_tree_crate_bundle(
        &self,
        crate_path: CratePath,
    ) -> EntityTreeCrateBundleResult<&EntityTreeCrateBundle> {
        Ok(entity_tree_crate_bundle(self, crate_path)
            .as_ref()
            .map_err(|e| e.clone())?)
    }

    fn entree_module_sheet(
        &self,
        module_path: ModulePath,
    ) -> EntityTreeResult<&EntityTreeModuleSheet> {
        module_entity_tree(self, module_path)
    }

    fn module_symbol_context<'a>(
        &'a self,
        module_path: ModulePath,
    ) -> EntityTreeResult<ModuleSymbolContext<'a>> {
        module_symbol_context(self, module_path)
    }

    fn subentity_path(
        &self,
        parent: EntityPath,
        identifier: Identifier,
    ) -> EntityTreeResult<EntityPath> {
        subentity_path(self, parent, identifier)
    }

    fn impl_block_associated_items(&self, impl_block: ImplBlock) -> &IdentPairMap<AssociatedItem> {
        impl_block_associated_items(self, impl_block)
    }
}
