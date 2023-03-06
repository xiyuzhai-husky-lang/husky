use std::sync::Arc;

use crate::*;
use husky_entity_path::EntityPathDb;
use husky_valid_ty::ValidTypeDb;
use salsa::DbWithJar;

pub trait TermDb: DbWithJar<TermJar> + ValidTypeDb {
    // fn term_menu(&self, toolchain: Toolchain) -> TermResultRef<&TermMenu>;
}

impl<Db> TermDb for Db
where
    Db: DbWithJar<TermJar> + ValidTypeDb,
{
    // fn term_menu(&self, toolchain: Toolchain) -> TermResultRef<&TermMenu> {
    //     term_menu(self, toolchain).as_ref()
    // }
}
