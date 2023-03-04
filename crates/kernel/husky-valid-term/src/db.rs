use std::sync::Arc;

use crate::*;
use husky_entity_path::EntityPathDb;
use husky_precise_term::PreciseTermDb;
use salsa::DbWithJar;

pub trait ValidTermDb: DbWithJar<ValidTermJar> + PreciseTermDb {
    fn valid_term_menu(&self, toolchain: Toolchain) -> &ValidTermResult<ValidTermMenu>;
}

impl<Db> ValidTermDb for Db
where
    Db: DbWithJar<ValidTermJar> + PreciseTermDb,
{
    fn valid_term_menu(&self, toolchain: Toolchain) -> &ValidTermResult<ValidTermMenu> {
        valid_term_menu(self, toolchain)
    }
}
