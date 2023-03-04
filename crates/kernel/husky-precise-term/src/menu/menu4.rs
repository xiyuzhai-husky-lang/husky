use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct PreciseTermMenu4 {
    parent: PreciseTermMenu3,
}

impl std::ops::Deref for PreciseTermMenu4 {
    type Target = PreciseTermMenu3;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl PreciseTermMenu4 {
    pub(super) fn new(
        _db: &dyn PreciseTermDb,
        _toolchain: Toolchain,
        menu3: PreciseTermMenu3,
    ) -> Self {
        Self { parent: menu3 }
    }
}
