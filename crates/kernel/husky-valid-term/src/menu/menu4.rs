use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ValidTermMenu4 {
    parent: ValidTermMenu3,
}

impl std::ops::Deref for ValidTermMenu4 {
    type Target = ValidTermMenu3;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl ValidTermMenu4 {
    pub(super) fn new(_db: &dyn ValidTermDb, _toolchain: Toolchain, menu3: ValidTermMenu3) -> Self {
        Self { parent: menu3 }
    }
}
