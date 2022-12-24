use crate::*;

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub struct TokenIdx(pub(crate) usize);

impl std::ops::Index<TokenIdx> for TokenSheet {
    type Output = Token;

    fn index(&self, index: TokenIdx) -> &Self::Output {
        &self.tokens[index.0]
    }
}
