use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TomlSectionTitle(SmallVec<[BaseCoword; 2]>);

impl TomlSectionTitle {
    pub(super) fn new(words: SmallVec<[BaseCoword; 2]>) -> Self {
        Self(words)
    }
}

impl std::ops::Deref for TomlSectionTitle {
    type Target = [BaseCoword];

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}
