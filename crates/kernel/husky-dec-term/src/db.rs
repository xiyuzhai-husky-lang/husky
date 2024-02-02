use crate::*;

use husky_vfs::Toolchain;

pub trait DeclarativeTermDb {
    fn declarative_term_menu(
        &self,
        toolchain: Toolchain,
    ) -> DeclarativeTermResultRef<&DeclarativeTermMenu>;
}

impl DeclarativeTermDb for ::salsa::Db {
    fn declarative_term_menu(
        &self,
        toolchain: Toolchain,
    ) -> DeclarativeTermResultRef<&DeclarativeTermMenu> {
        declarative_term_menu(self, toolchain).as_ref()
    }
}
