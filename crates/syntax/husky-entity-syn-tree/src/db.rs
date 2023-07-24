use crate::*;

use husky_ast::AstDb;

use husky_entity_path::ItemPath;
use husky_entity_taxonomy::EntityKind;
use husky_manifest::ManifestDb;
use husky_vfs::*;

use salsa::DbWithJar;

pub trait EntitySynTreeDb: DbWithJar<EntitySynTreeJar> + AstDb + EntityPathDb + ManifestDb {
    fn submodules(&self, module_path: ModulePath) -> VfsResult<&[SubmodulePath]>;
    fn all_modules_within_crate(&self, crate_path: CratePath) -> &[ModulePath];
    fn item_syn_tree_bundle(
        &self,
        crate_path: CratePath,
    ) -> EntitySynTreeBundleResult<&EntitySynTreeCrateBundle>;
    fn item_syn_tree_presheet(&self, module_path: ModulePath) -> VfsResult<&EntitySynTreePresheet>;
    fn item_syn_tree_sheet(
        &self,
        module_path: ModulePath,
    ) -> ItemSynTreeResult<&EntitySynTreeSheet>;
    fn module_symbol_context<'a>(
        &'a self,
        module_path: ModulePath,
    ) -> ItemSynTreeResult<ModuleSymbolContext<'a>>;
    fn subitem_path(
        &self,
        parent: MajorEntityPath,
        identifier: Ident,
    ) -> ItemSynTreeResult<SubitemPath>;
}

impl<T> EntitySynTreeDb for T
where
    T: DbWithJar<EntitySynTreeJar> + AstDb + EntityPathDb + ManifestDb,
{
    fn submodules(&self, module_path: ModulePath) -> VfsResult<&[SubmodulePath]> {
        Ok(submodules(self, module_path).as_ref()?)
    }

    fn all_modules_within_crate(&self, crate_path: CratePath) -> &[ModulePath] {
        all_modules_within_crate(self, crate_path)
    }

    fn item_syn_tree_bundle(
        &self,
        crate_path: CratePath,
    ) -> EntitySynTreeBundleResult<&EntitySynTreeCrateBundle> {
        Ok(item_tree_crate_bundle(self, crate_path)
            .as_ref()
            .map_err(|e| e.clone())?)
    }

    fn item_syn_tree_presheet(&self, module_path: ModulePath) -> VfsResult<&EntitySynTreePresheet> {
        Ok(item_tree_presheet(self, module_path).as_ref()?)
    }

    fn item_syn_tree_sheet(
        &self,
        module_path: ModulePath,
    ) -> ItemSynTreeResult<&EntitySynTreeSheet> {
        item_tree_sheet(self, module_path)
    }

    fn module_symbol_context<'a>(
        &'a self,
        module_path: ModulePath,
    ) -> ItemSynTreeResult<ModuleSymbolContext<'a>> {
        module_symbol_context(self, module_path)
    }

    fn subitem_path(
        &self,
        parent: MajorEntityPath,
        identifier: Ident,
    ) -> ItemSynTreeResult<SubitemPath> {
        subitem_path(self, parent, identifier)
    }
}
