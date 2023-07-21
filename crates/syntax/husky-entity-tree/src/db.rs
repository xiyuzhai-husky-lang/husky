use crate::*;

use husky_ast::AstDb;

use husky_entity_path::EntityPath;
use husky_entity_taxonomy::EntityKind;
use husky_manifest::ManifestDb;
use husky_vfs::*;

use salsa::DbWithJar;

pub trait EntitySynTreeDb: DbWithJar<EntitySynTreeJar> + AstDb + EntityPathDb + ManifestDb {
    fn submodules(&self, module_path: ModulePath) -> VfsResult<&[ModulePath]>;
    fn all_modules_within_crate(&self, crate_path: CratePath) -> &[ModulePath];
    fn entity_syn_tree_bundle(
        &self,
        crate_path: CratePath,
    ) -> EntitySynTreeBundleResult<&EntitySynTreeCrateBundle>;
    fn entity_syn_tree_presheet(
        &self,
        module_path: ModulePath,
    ) -> VfsResult<&EntitySynTreePresheet>;
    fn entity_syn_tree_sheet(
        &self,
        module_path: ModulePath,
    ) -> EntitySynTreeResult<&EntitySynTreeSheet>;
    fn module_symbol_context<'a>(
        &'a self,
        module_path: ModulePath,
    ) -> EntitySynTreeResult<ModuleSymbolContext<'a>>;
    fn subentity_path(
        &self,
        parent: MajorEntityPath,
        identifier: Ident,
    ) -> EntitySynTreeResult<SubentityPath>;
}

impl<T> EntitySynTreeDb for T
where
    T: DbWithJar<EntitySynTreeJar> + AstDb + EntityPathDb + ManifestDb,
{
    fn submodules(&self, module_path: ModulePath) -> VfsResult<&[ModulePath]> {
        Ok(submodules(self, module_path).as_ref()?)
    }

    fn all_modules_within_crate(&self, crate_path: CratePath) -> &[ModulePath] {
        all_modules_within_crate(self, crate_path)
    }

    fn entity_syn_tree_bundle(
        &self,
        crate_path: CratePath,
    ) -> EntitySynTreeBundleResult<&EntitySynTreeCrateBundle> {
        Ok(entity_tree_crate_bundle(self, crate_path)
            .as_ref()
            .map_err(|e| e.clone())?)
    }

    fn entity_syn_tree_presheet(
        &self,
        module_path: ModulePath,
    ) -> VfsResult<&EntitySynTreePresheet> {
        Ok(entity_tree_presheet(self, module_path).as_ref()?)
    }

    fn entity_syn_tree_sheet(
        &self,
        module_path: ModulePath,
    ) -> EntitySynTreeResult<&EntitySynTreeSheet> {
        entity_tree_sheet(self, module_path)
    }

    fn module_symbol_context<'a>(
        &'a self,
        module_path: ModulePath,
    ) -> EntitySynTreeResult<ModuleSymbolContext<'a>> {
        module_symbol_context(self, module_path)
    }

    fn subentity_path(
        &self,
        parent: MajorEntityPath,
        identifier: Ident,
    ) -> EntitySynTreeResult<SubentityPath> {
        subentity_path(self, parent, identifier)
    }
}
