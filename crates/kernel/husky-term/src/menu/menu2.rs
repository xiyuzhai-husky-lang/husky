use crate::*;
use husky_toolchain::Toolchain;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu2 {
    parent: TermMenu1,
}

impl std::ops::Deref for TermMenu2 {
    type Target = TermMenu1;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl TermMenu2 {
    pub(crate) fn new(db: &dyn TermDb, toolchain: Toolchain, menu1: TermMenu1) -> Self {
        db.it_entity_path_term(db.entity_path_menu(toolchain).b32());
        TermMenu2 { parent: menu1 }
    }
}
