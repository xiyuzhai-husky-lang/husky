use crate::*;
use husky_coword::CowordDb;

pub trait EntityPathDb: DbWithJar<EntityPathJar> + VfsDb + CowordDb {
    fn item_path_menu(&self, toolchain: Toolchain) -> &EntityPathMenu;
}

impl<T> EntityPathDb for T
where
    T: DbWithJar<EntityPathJar> + VfsDb + CowordDb,
{
    fn item_path_menu(&self, toolchain: Toolchain) -> &EntityPathMenu {
        item_path_menu(self, toolchain)
    }
}
