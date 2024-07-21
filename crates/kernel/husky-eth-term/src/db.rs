use super::*;

pub trait EthTermDb {
    fn eth_term_menu(&self, toolchain: Toolchain) -> &EthTermMenu;
}

impl EthTermDb for ::salsa::Db {
    fn eth_term_menu(&self, toolchain: Toolchain) -> &EthTermMenu {
        term_menu(self, toolchain)
    }
}
