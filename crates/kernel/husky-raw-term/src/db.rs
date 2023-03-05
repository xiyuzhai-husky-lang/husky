use std::sync::Arc;

use crate::*;
use husky_entity_path::EntityPathDb;
use husky_vfs::Toolchain;
use salsa::DbWithJar;

pub trait RawTermDb: DbWithJar<RawTermJar> + EntityPathDb {
    fn raw_term_menu(&self, toolchain: Toolchain) -> RawTermResultRef<&RawTermMenu>;
}

impl<Db> RawTermDb for Db
where
    Db: DbWithJar<RawTermJar> + EntityPathDb,
{
    fn raw_term_menu(&self, toolchain: Toolchain) -> RawTermResultRef<&RawTermMenu> {
        raw_term_menu(self, toolchain).as_ref()
    }
}
