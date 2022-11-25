use crate::Token;
pub struct TokenStorage(Vec<Token>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenGroup(std::ops::Range<usize>);

impl TokenGroup {
    pub fn new(range: std::ops::Range<usize>) -> Self {
        Self(range)
    }

    pub fn first(&self, storage: &TokenStorage) -> &Token {
        todo!()
    }
}

impl std::ops::Index<TokenGroup> for TokenStorage {
    type Output = [Token];

    fn index(&self, index: TokenGroup) -> &Self::Output {
        &self.0[index.0]
    }
}

impl std::ops::Deref for TokenGroup {
    type Target = std::ops::Range<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
