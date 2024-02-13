use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenVerseStart(TokenIdx);

impl TokenVerseStart {
    pub fn from_index(index: usize) -> Self {
        Self(TokenIdx::from_index(index))
    }

    pub fn token_idx(self) -> TokenIdx {
        self.0
    }

    pub fn index(self) -> usize {
        self.0.index()
    }
}
