use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ValidTermMenu5 {
    parent: ValidTermMenu4,
}

impl std::ops::Deref for ValidTermMenu5 {
    type Target = ValidTermMenu4;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl ValidTermMenu5 {
    pub(super) fn new(_db: &dyn ValidTermDb, _toolchain: Toolchain, menu4: ValidTermMenu4) -> Self {
        // let _core_ops = menu4.core_ops();
        Self { parent: menu4 }
    }
}
