use super::*;

pub trait EthTermDb {
    fn ethereal_term_menu(&self, toolchain: Toolchain) -> &EthTermMenu;
}

impl EthTermDb for ::salsa::Db {
    fn ethereal_term_menu(&self, toolchain: Toolchain) -> &EthTermMenu {
        term_menu(self, toolchain)
    }
}
