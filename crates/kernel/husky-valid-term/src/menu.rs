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
pub struct ValidTermMenu {
    parent: ValidTermMenu5,
}

impl std::ops::Deref for ValidTermMenu {
    type Target = ValidTermMenu5;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl ValidTermMenu {
    fn new(db: &dyn ValidTermDb, toolchain: Toolchain) -> ValidTermResult<ValidTermMenu> {
        let menu0 = ValidTermMenu0::new(db, toolchain);
        let menu1 = ValidTermMenu1::new(db, toolchain, menu0);
        let menu2 = ValidTermMenu2::new(db, toolchain, menu1)?;
        let menu3 = ValidTermMenu3::new(db, toolchain, menu2)?;
        let menu4 = ValidTermMenu4::new(db, toolchain, menu3);
        let menu5 = ValidTermMenu5::new(db, toolchain, menu4);
        Ok(ValidTermMenu { parent: menu5 })
    }
}

#[salsa::tracked(jar = ValidTermJar,return_ref)]
pub(crate) fn valid_term_menu(
    db: &dyn ValidTermDb,
    toolchain: Toolchain,
) -> ValidTermResult<ValidTermMenu> {
    ValidTermMenu::new(db, toolchain)
}
