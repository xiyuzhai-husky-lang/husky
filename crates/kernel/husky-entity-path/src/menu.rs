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
    pub(crate) fn new(db: &dyn EntityPathDb) -> Self {
        let menu0 = EntityPathMenu0::new(db);
        let menu1 = EntityPathMenu1::new(db, menu0);
        let menu2 = EntityPathMenu2::new(db, menu1);
        let menu3 = EntityPathMenu3::new(db, menu2);
        Self { parent: menu3 }
    }
}
