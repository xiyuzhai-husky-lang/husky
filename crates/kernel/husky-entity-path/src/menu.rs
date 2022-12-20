mod menu0;
mod menu1;
mod menu2;
mod menu3;

use crate::*;
use menu0::*;
use menu1::*;
use menu2::*;
use menu3::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPathMenu {
    parent: EntityPathMenu3,
}

impl std::ops::Deref for EntityPathMenu {
    type Target = EntityPathMenu3;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl EntityPathMenu {
    pub(crate) fn new(db: &dyn EntityPathDb, toolchain: Toolchain) -> EntityPathResult<Self> {
        let menu0 = EntityPathMenu0::new(db, toolchain)?;
        let menu1 = EntityPathMenu1::new(db, toolchain, menu0);
        let menu2 = EntityPathMenu2::new(db, toolchain, menu1);
        let menu3 = EntityPathMenu3::new(db, toolchain, menu2);
        Ok(Self { parent: menu3 })
    }
}

#[salsa::tracked(jar = EntityPathJar, return_ref)]
pub(crate) fn entity_path_menu(
    db: &dyn EntityPathDb,
    toolchain: Toolchain,
) -> EntityPathResult<EntityPathMenu> {
    EntityPathMenu::new(db, toolchain)
}
