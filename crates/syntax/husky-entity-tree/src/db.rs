use crate::*;

use husky_ast::AstDb;

use husky_entity_card::EntityCard;
use husky_entity_path::EntityPath;
use husky_vfs::*;

use salsa::DbWithJar;

pub trait EntityTreeDb: DbWithJar<EntityTreeJar> + AstDb + EntityPathDb {
    fn entity_tree_sheet(&self, module_path: ModulePath) -> VfsResult<&EntityTreeSheet>;
    fn entity_card(&self, entity_path: EntityPath) -> EntityTreeResult<EntityCard> {
        match entity_path {
            EntityPath::Module(_) => todo!(),
            EntityPath::ModuleItem(_) => todo!(),
            EntityPath::AssociatedItem(_) => todo!(),
        }
    }
    fn module_item_entity_card(
        &self,
        module_item_path: ModuleItemPath,
    ) -> &EntityTreeResult<EntityCard>;
    fn associated_item_entity_card(
        &self,
        associated_item_path: AssociatedItemPath,
    ) -> &EntityTreeResult<EntityCard>;
    fn submodules(&self, module_path: ModulePath) -> VfsResult<&[ModulePath]>;
    fn all_modules_within_crate(&self, crate_path: CratePath) -> VfsResult<&[ModulePath]>;
}

impl<T> EntityTreeDb for T
where
    T: DbWithJar<EntityTreeJar> + AstDb + EntityPathDb,
{
    fn entity_tree_sheet(&self, module_path: ModulePath) -> VfsResult<&EntityTreeSheet> {
        Ok(entity_tree_sheet(self, module_path).as_ref()?)
    }
    fn module_item_entity_card(
        &self,
        module_item_path: ModuleItemPath,
    ) -> &EntityTreeResult<EntityCard> {
        todo!()
    }
    fn associated_item_entity_card(
        &self,
        associated_item_path: AssociatedItemPath,
    ) -> &EntityTreeResult<EntityCard> {
        todo!()
    }

    fn submodules(&self, module_path: ModulePath) -> VfsResult<&[ModulePath]> {
        Ok(submodules(self, module_path).as_ref()?)
    }

    fn all_modules_within_crate(&self, crate_path: CratePath) -> VfsResult<&[ModulePath]> {
        Ok(all_modules_within_crate(self, crate_path).as_ref()?)
    }
}
