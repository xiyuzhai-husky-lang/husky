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

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EthTermMenu {
    parent: TermMenu5,
}

impl std::ops::Deref for EthTermMenu {
    type Target = TermMenu5;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl EthTermMenu {
    fn new(db: &::salsa::Db, toolchain: Toolchain) -> EthTermMenu {
        let menu0 = TermMenu0::new(db, toolchain);
        let menu1 = TermMenu1::new(db, toolchain, menu0);
        let menu2 = TermMenu2::new(db, toolchain, menu1);
        let menu3 = TermMenu3::new(db, toolchain, menu2);
        let menu4 = TermMenu4::new(db, toolchain, menu3);
        let menu5 = TermMenu5::new(db, toolchain, menu4);
        EthTermMenu { parent: menu5 }
    }
}

#[salsa::tracked(return_ref)]
pub(crate) fn term_menu(db: &::salsa::Db, toolchain: Toolchain) -> EthTermMenu {
    EthTermMenu::new(db, toolchain)
}
