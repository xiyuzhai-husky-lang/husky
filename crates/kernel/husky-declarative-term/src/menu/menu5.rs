use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DeclarativeTermMenu5 {
    parent: DeclarativeTermMenu4,
}

impl std::ops::Deref for DeclarativeTermMenu5 {
    type Target = DeclarativeTermMenu4;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DeclarativeTermMenu5 {
    pub(super) fn new(
        _db: &dyn DeclarativeTermDb,
        _toolchain: Toolchain,
        menu4: DeclarativeTermMenu4,
    ) -> Self {
        // let _core_ops = menu4.core_ops();
        Self { parent: menu4 }
    }
}
