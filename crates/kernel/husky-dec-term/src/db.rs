use crate::*;

use husky_vfs::Toolchain;

pub trait DecTermDb {
    fn declarative_term_menu(&self, toolchain: Toolchain) -> DecTermResultRef<&DecTermMenu>;
}

impl DecTermDb for ::salsa::Db {
    fn declarative_term_menu(&self, toolchain: Toolchain) -> DecTermResultRef<&DecTermMenu> {
        declarative_term_menu(self, toolchain).as_ref()
    }
}
