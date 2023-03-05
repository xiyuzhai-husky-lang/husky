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
pub struct PreciseTermMenu {
    parent: PreciseTermMenu5,
}

impl std::ops::Deref for PreciseTermMenu {
    type Target = PreciseTermMenu5;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl PreciseTermMenu {
    fn new(db: &dyn PreciseTermDb, toolchain: Toolchain) -> PreciseTermResult<PreciseTermMenu> {
        let menu0 = PreciseTermMenu0::new(db, toolchain);
        let menu1 = PreciseTermMenu1::new(db, toolchain, menu0);
        let menu2 = PreciseTermMenu2::new(db, toolchain, menu1)?;
        let menu3 = PreciseTermMenu3::new(db, toolchain, menu2)?;
        let menu4 = PreciseTermMenu4::new(db, toolchain, menu3);
        let menu5 = PreciseTermMenu5::new(db, toolchain, menu4);
        Ok(PreciseTermMenu { parent: menu5 })
    }
}

#[salsa::tracked(jar = PreciseTermJar, return_ref)]
pub(crate) fn precise_term_menu(
    db: &dyn PreciseTermDb,
    toolchain: Toolchain,
) -> PreciseTermResult<PreciseTermMenu> {
    PreciseTermMenu::new(db, toolchain)
}
