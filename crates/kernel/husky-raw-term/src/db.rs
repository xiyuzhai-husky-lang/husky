use crate::*;

use husky_term_prelude::TermPreludeDb;
use husky_vfs::Toolchain;
use salsa::DbWithJar;

pub trait RawTermDb: DbWithJar<RawTermJar> + TermPreludeDb {
    fn raw_term_menu(&self, toolchain: Toolchain) -> RawTermResultRef<&RawTermMenu>;
}

impl<Db> RawTermDb for Db
where
    Db: DbWithJar<RawTermJar> + TermPreludeDb,
{
    fn raw_term_menu(&self, toolchain: Toolchain) -> RawTermResultRef<&RawTermMenu> {
        raw_term_menu(self, toolchain).as_ref()
    }
}
