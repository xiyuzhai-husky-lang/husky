use crate::*;

use husky_vfs::Toolchain;

pub trait DecTermDb {
    fn dec_term_menu(&self, toolchain: Toolchain) -> DecTermResultRef<&DecTermMenu>;
}

impl DecTermDb for ::salsa::Db {
    fn dec_term_menu(&self, toolchain: Toolchain) -> DecTermResultRef<&DecTermMenu> {
        dec_term_menu(self, toolchain).as_ref()
    }
}
