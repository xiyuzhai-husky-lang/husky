use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu4 {
    parent: TermMenu3,
}

impl std::ops::Deref for TermMenu4 {
    type Target = TermMenu3;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl TermMenu4 {
    pub(super) fn new(db: &dyn TermDb, toolchain: Toolchain, menu3: TermMenu3) -> Self {
        Self { parent: menu3 }
    }
}
