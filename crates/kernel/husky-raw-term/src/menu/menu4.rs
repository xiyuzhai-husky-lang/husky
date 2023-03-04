use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct RawTermMenu4 {
    parent: RawTermMenu3,
}

impl std::ops::Deref for RawTermMenu4 {
    type Target = RawTermMenu3;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl RawTermMenu4 {
    pub(super) fn new(_db: &dyn RawTermDb, _toolchain: Toolchain, menu3: RawTermMenu3) -> Self {
        Self { parent: menu3 }
    }
}
