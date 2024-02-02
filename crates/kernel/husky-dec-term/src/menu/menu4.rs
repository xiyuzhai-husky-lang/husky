use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DeclarativeTermMenu4 {
    parent: DeclarativeTermMenu3,
}

impl std::ops::Deref for DeclarativeTermMenu4 {
    type Target = DeclarativeTermMenu3;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DeclarativeTermMenu4 {
    pub(super) fn new(
        _db: &::salsa::Db,
        _toolchain: Toolchain,
        menu3: DeclarativeTermMenu3,
    ) -> Self {
        Self { parent: menu3 }
    }
}
