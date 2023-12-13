use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TomlSectionTitle(SmallVec<[Coword; 2]>);

impl TomlSectionTitle {
    pub(super) fn new(words: SmallVec<[Coword; 2]>) -> Self {
        Self(words)
    }
}

impl std::ops::Deref for TomlSectionTitle {
    type Target = [Coword];

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}
