use crate::*;
use std::sync::Arc;

pub trait EtherealTermDb {
    fn ethereal_term_menu(&self, toolchain: Toolchain) -> &EtherealTermMenu;
}

impl EtherealTermDb for ::salsa::Db {
    fn ethereal_term_menu(&self, toolchain: Toolchain) -> &EtherealTermMenu {
        term_menu(self, toolchain)
    }
}
