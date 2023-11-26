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
pub struct DeclarativeTermMenu {
    parent: DeclarativeTermMenu5,
}

impl std::ops::Deref for DeclarativeTermMenu {
    type Target = DeclarativeTermMenu5;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DeclarativeTermMenu {
    fn new(db: &::salsa::Db, toolchain: Toolchain) -> DeclarativeTermResult<DeclarativeTermMenu> {
        let menu0 = DeclarativeTermMenu0::new(db, toolchain);
        let menu1 = DeclarativeTermMenu1::new(db, toolchain, menu0);
        let menu2 = DeclarativeTermMenu2::new(db, toolchain, menu1)?;
        let menu3 = DeclarativeTermMenu3::new(db, toolchain, menu2)?;
        let menu4 = DeclarativeTermMenu4::new(db, toolchain, menu3);
        let menu5 = DeclarativeTermMenu5::new(db, toolchain, menu4);
        Ok(DeclarativeTermMenu { parent: menu5 })
    }
}

#[salsa::tracked(jar = DeclarativeTermJar, return_ref)]
pub(crate) fn declarative_term_menu(
    db: &::salsa::Db,
    toolchain: Toolchain,
) -> DeclarativeTermResult<DeclarativeTermMenu> {
    DeclarativeTermMenu::new(db, toolchain)
}
