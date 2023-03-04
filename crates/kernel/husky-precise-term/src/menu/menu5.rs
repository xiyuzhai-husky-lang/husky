use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct PreciseTermMenu5 {
    parent: PreciseTermMenu4,
}

impl std::ops::Deref for PreciseTermMenu5 {
    type Target = PreciseTermMenu4;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl PreciseTermMenu5 {
    pub(super) fn new(
        _db: &dyn PreciseTermDb,
        _toolchain: Toolchain,
        menu4: PreciseTermMenu4,
    ) -> Self {
        // let _core_ops = menu4.core_ops();
        Self { parent: menu4 }
    }
}
