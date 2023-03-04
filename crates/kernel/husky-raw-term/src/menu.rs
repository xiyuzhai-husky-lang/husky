mod menu0;
mod menu1;
mod menu2;
mod menu3;
mod menu4;
mod menu5;

pub use menu0::*;
pub use menu1::*;
pub use menu2::*;
pub use menu3::*;
pub use menu4::*;
pub use menu5::*;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct RawTermMenu {
    parent: RawTermMenu5,
}

impl std::ops::Deref for RawTermMenu {
    type Target = RawTermMenu5;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl RawTermMenu {
    fn new(db: &dyn RawTermDb, toolchain: Toolchain) -> RawTermResult<RawTermMenu> {
        let menu0 = RawTermMenu0::new(db, toolchain);
        let menu1 = RawTermMenu1::new(db, toolchain, menu0);
        let menu2 = RawTermMenu2::new(db, toolchain, menu1)?;
        let menu3 = RawTermMenu3::new(db, toolchain, menu2)?;
        let menu4 = RawTermMenu4::new(db, toolchain, menu3);
        let menu5 = RawTermMenu5::new(db, toolchain, menu4);
        Ok(RawTermMenu { parent: menu5 })
    }
}

#[salsa::tracked(jar = RawTermJar,return_ref)]
pub(crate) fn raw_term_menu(
    db: &dyn RawTermDb,
    toolchain: Toolchain,
) -> RawTermResult<RawTermMenu> {
    RawTermMenu::new(db, toolchain)
}
