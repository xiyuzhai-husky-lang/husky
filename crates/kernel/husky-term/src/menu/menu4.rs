use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu4 {
    core_ops: TermItd,
    parent: TermMenu3,
}

impl std::ops::Deref for TermMenu4 {
    type Target = TermMenu3;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl TermMenu4 {
    pub(super) fn new(db: &dyn TermDb, menu3: TermMenu3) -> Self {
        Self {
            core_ops: TermSubentity::new(db, menu3.core(), "ops"),
            parent: menu3,
        }
    }

    pub fn core_ops(&self) -> TermItd {
        self.core_ops
    }
}
