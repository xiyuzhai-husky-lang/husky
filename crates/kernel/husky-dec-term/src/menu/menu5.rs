use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DecTermMenu5 {
    parent: DecTermMenu4,
}

impl std::ops::Deref for DecTermMenu5 {
    type Target = DecTermMenu4;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DecTermMenu5 {
    pub(super) fn new(_db: &::salsa::Db, _toolchain: Toolchain, menu4: DecTermMenu4) -> Self {
        // let _core_ops = menu4.core_ops();
        Self { parent: menu4 }
    }
}
