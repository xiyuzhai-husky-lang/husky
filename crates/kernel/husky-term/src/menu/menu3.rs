use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu3 {
    parent: TermMenu2,
}

impl std::ops::Deref for TermMenu3 {
    type Target = TermMenu2;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl TermMenu3 {
    pub(crate) fn new(db: &dyn TermDb, toolchain: Toolchain, menu2: TermMenu2) -> Self {
        let menu0 = TermMenu0::new(db, toolchain);
        let menu1 = TermMenu1::new(db, toolchain, menu0);
        let menu2 = TermMenu2::new(db, toolchain, menu1);
        TermMenu3 { parent: menu2 }
    }
}

type A = core::primitive::i32;
