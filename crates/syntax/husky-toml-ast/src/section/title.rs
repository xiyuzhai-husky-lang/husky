use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TomlSectionTitle(SmallVec<[Word; 2]>);

impl TomlSectionTitle {
    pub(super) fn new(words: SmallVec<[Word; 2]>) -> Self {
        Self(words)
    }
}

impl std::ops::Deref for TomlSectionTitle {
    type Target = [Word];

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}
