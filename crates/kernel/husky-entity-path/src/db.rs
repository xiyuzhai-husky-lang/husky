use crate::*;
use husky_word::WordDb;

pub trait EntityPathDb: DbWithJar<EntityPathJar> + VfsDb + WordDb {
    fn entity_path_menu(&self, toolchain: Toolchain) -> EntityPathResult<&EntityPathMenu>;
    fn apparent_crate_of_entity_path(&self, path: EntityPath) -> CratePath;
    fn entity_apparent_ancestry(&self, path: EntityPath) -> &EntityAncestry;
}

impl<T> EntityPathDb for T
where
    T: DbWithJar<EntityPathJar> + VfsDb + WordDb,
{
    fn entity_path_menu(&self, toolchain: Toolchain) -> EntityPathResult<&EntityPathMenu> {
        Ok(entity_path_menu(self, toolchain).as_ref()?)
    }

    fn apparent_crate_of_entity_path(&self, entity_path: EntityPath) -> CratePath {
        self.entity_apparent_ancestry(entity_path).crate_path()
    }

    fn entity_apparent_ancestry(&self, path: EntityPath) -> &EntityAncestry {
        entity_apparent_ancestry(self, path)
    }
}
