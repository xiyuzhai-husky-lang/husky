mod menu0;
mod menu1;
mod menu2;
mod menu3;
mod menu4;
mod menu5;

pub use self::menu0::*;
pub use self::menu1::*;
pub use self::menu2::*;
pub use self::menu3::*;
pub use self::menu4::*;
pub use self::menu5::*;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DecTermMenu {
    parent: DecTermMenu5,
}

impl std::ops::Deref for DecTermMenu {
    type Target = DecTermMenu5;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DecTermMenu {
    fn new(db: &::salsa::Db, toolchain: Toolchain) -> DecTermResult<DecTermMenu> {
        let menu0 = DecTermMenu0::new(db, toolchain);
        let menu1 = DecTermMenu1::new(db, toolchain, menu0);
        let menu2 = DecTermMenu2::new(db, toolchain, menu1)?;
        let menu3 = DecTermMenu3::new(db, toolchain, menu2)?;
        let menu4 = DecTermMenu4::new(db, toolchain, menu3);
        let menu5 = DecTermMenu5::new(db, toolchain, menu4);
        Ok(DecTermMenu { parent: menu5 })
    }
}

#[salsa::tracked(jar = DecTermJar, return_ref)]
pub(crate) fn declarative_term_menu(
    db: &::salsa::Db,
    toolchain: Toolchain,
) -> DecTermResult<DecTermMenu> {
    DecTermMenu::new(db, toolchain)
}
