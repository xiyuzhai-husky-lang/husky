use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu3 {
    core: TermItd,
    parent: TermMenu2,
}

impl std::ops::Deref for TermMenu3 {
    type Target = TermMenu2;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl TermMenu3 {
    pub(crate) fn new(db: &dyn TermDb, menu2: TermMenu2) -> Self {
        let menu0 = TermMenu0::new(db);
        let menu1 = TermMenu1::new(db, menu0);
        let menu2 = TermMenu2::new(db, menu1);
        TermMenu3 {
            core: Term::core(db, &menu2),
            parent: menu2,
        }
    }

    pub fn core(&self) -> TermItd {
        self.core
    }
}
type A = core::primitive::i32;
