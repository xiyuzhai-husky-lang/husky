use std::sync::Arc;

use crate::*;
use husky_declarative_ty::DeclarativeTypeDb;
use husky_entity_path::EntityPathDb;
use salsa::DbWithJar;

pub trait EtherealTermDb: DbWithJar<EtherealTermJar> + DeclarativeTypeDb {
    fn ethereal_term_menu(&self, toolchain: Toolchain) -> &EtherealTermMenu;
}

impl<Db> EtherealTermDb for Db
where
    Db: DbWithJar<EtherealTermJar> + DeclarativeTypeDb,
{
    fn ethereal_term_menu(&self, toolchain: Toolchain) -> &EtherealTermMenu {
        term_menu(self, toolchain)
    }
}
