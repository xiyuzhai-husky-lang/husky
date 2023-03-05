use std::sync::Arc;

use crate::*;
use husky_entity_path::EntityPathDb;
use husky_valid_term::ValidTermDb;
use salsa::DbWithJar;

pub trait TermDb: DbWithJar<TermJar> + ValidTermDb {
    fn term_menu(&self, toolchain: Toolchain) -> TermResultRef<&TermMenu>;
}

impl<Db> TermDb for Db
where
    Db: DbWithJar<TermJar> + ValidTermDb,
{
    fn term_menu(&self, toolchain: Toolchain) -> TermResultRef<&TermMenu> {
        term_menu(self, toolchain).as_ref()
    }
}
