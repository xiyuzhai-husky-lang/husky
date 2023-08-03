use crate::*;
use husky_coword::CowordDb;

pub trait EntityPathDb: DbWithJar<EntityPathJar> + VfsDb + CowordDb {
    fn item_path_menu(&self, toolchain: Toolchain) -> &ItemPathMenu;
}

impl<T> EntityPathDb for T
where
    T: DbWithJar<EntityPathJar> + VfsDb + CowordDb,
{
    fn item_path_menu(&self, toolchain: Toolchain) -> &ItemPathMenu {
        item_path_menu(self, toolchain)
    }
}
