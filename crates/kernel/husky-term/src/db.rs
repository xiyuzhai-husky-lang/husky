use std::sync::Arc;

use crate::*;
use husky_entity_path::EntityPathDb;

use salsa::DbWithJar;

pub trait TermDb: DbWithJar<TermJar> + EntityPathDb {
    fn term_menu(&self, toolchain: Toolchain) -> &TermResult<TermMenu>;
}

impl<Db> TermDb for Db
where
    Db: DbWithJar<TermJar> + EntityPathDb,
{
    fn term_menu(&self, toolchain: Toolchain) -> &TermResult<TermMenu> {
        term_menu(self, toolchain)
    }
}
