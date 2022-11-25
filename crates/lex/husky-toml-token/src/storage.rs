use crate::TomlToken;
pub struct TomlTokenStorage(Vec<TomlToken>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TomlTokenGroup(std::ops::Range<usize>);

impl TomlTokenGroup {
    pub fn new(range: std::ops::Range<usize>) -> Self {
        Self(range)
    }

    pub fn first(&self, storage: &TomlTokenStorage) -> &TomlToken {
        todo!()
    }
}

impl std::ops::Index<TomlTokenGroup> for TomlTokenStorage {
    type Output = [TomlToken];

    fn index(&self, index: TomlTokenGroup) -> &Self::Output {
        &self.0[index.0]
    }
}

impl std::ops::Deref for TomlTokenGroup {
    type Target = std::ops::Range<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
