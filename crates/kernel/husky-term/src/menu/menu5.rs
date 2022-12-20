use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu5 {
    parent: TermMenu4,
}

impl std::ops::Deref for TermMenu5 {
    type Target = TermMenu4;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl TermMenu5 {
    pub(super) fn new(_db: &dyn TermDb, _toolchain: Toolchain, menu4: TermMenu4) -> Self {
        let _core_ops = menu4.core_ops();
        Self { parent: menu4 }
    }
}
