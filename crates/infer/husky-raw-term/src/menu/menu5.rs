use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct RawTermMenu5 {
    parent: RawTermMenu4,
}

impl std::ops::Deref for RawTermMenu5 {
    type Target = RawTermMenu4;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl RawTermMenu5 {
    pub(super) fn new(_db: &dyn RawTermDb, _toolchain: Toolchain, menu4: RawTermMenu4) -> Self {
        // let _core_ops = menu4.core_ops();
        Self { parent: menu4 }
    }
}
