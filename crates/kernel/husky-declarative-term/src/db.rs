use crate::*;

use husky_term_prelude::TermPreludeDb;
use husky_vfs::Toolchain;
use salsa::DbWithJar;

pub trait DeclarativeTermDb: DbWithJar<DeclarativeTermJar> + TermPreludeDb {
    fn raw_term_menu(&self, toolchain: Toolchain)
        -> DeclarativeTermResultRef<&DeclarativeTermMenu>;
}

impl<Db> DeclarativeTermDb for Db
where
    Db: DbWithJar<DeclarativeTermJar> + TermPreludeDb,
{
    fn raw_term_menu(
        &self,
        toolchain: Toolchain,
    ) -> DeclarativeTermResultRef<&DeclarativeTermMenu> {
        raw_term_menu(self, toolchain).as_ref()
    }
}
