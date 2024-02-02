use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DecTermMenu4 {
    parent: DecTermMenu3,
}

impl std::ops::Deref for DecTermMenu4 {
    type Target = DecTermMenu3;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DecTermMenu4 {
    pub(super) fn new(_db: &::salsa::Db, _toolchain: Toolchain, menu3: DecTermMenu3) -> Self {
        Self { parent: menu3 }
    }
}
