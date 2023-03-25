use crate::*;
use husky_word::WordDb;

pub trait EntityPathDb: DbWithJar<EntityPathJar> + VfsDb + WordDb {
    fn entity_path_menu(&self, toolchain: Toolchain) -> &EntityPathMenu;
}

impl<T> EntityPathDb for T
where
    T: DbWithJar<EntityPathJar> + VfsDb + WordDb,
{
    fn entity_path_menu(&self, toolchain: Toolchain) -> &EntityPathMenu {
        entity_path_menu(self, toolchain)
    }
}
