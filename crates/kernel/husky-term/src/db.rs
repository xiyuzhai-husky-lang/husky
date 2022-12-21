use std::sync::Arc;

use crate::*;
use husky_entity_path::EntityPathDb;

use salsa::DbWithJar;

pub trait TermDb: DbWithJar<TermJar> + EntityPathDb {
    fn term_menu(&self, toolchain: Toolchain) -> &TermResult<TermMenu>;
}

#[derive(Default)]
pub struct TermMenuPlace(Arc<once_cell::sync::OnceCell<TermMenu>>);

impl<T> TermDb for T
where
    T: DbWithJar<TermJar> + EntityPathDb,
{
    fn term_menu(&self, toolchain: Toolchain) -> &TermResult<TermMenu> {
        term_menu(self, toolchain)
    }
}
