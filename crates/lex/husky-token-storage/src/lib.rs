use husky_token::Token;
pub struct TokenStorage(Vec<Token>);

pub struct TokenIdxRange(pub std::ops::Range<usize>);

impl std::ops::Index<TokenIdxRange> for TokenStorage {
    type Output = [Token];

    fn index(&self, index: TokenIdxRange) -> &Self::Output {
        &self.0[index.0]
    }
}
