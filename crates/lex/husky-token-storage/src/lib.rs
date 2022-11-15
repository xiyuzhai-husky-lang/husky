use husky_token::Token;
pub struct TokenStorage(Vec<Token>);

pub struct TokenRange(pub std::ops::Range<usize>);

impl std::ops::Index<TokenRange> for TokenStorage {
    type Output = [Token];

    fn index(&self, index: TokenRange) -> &Self::Output {
        &self.0[index.0]
    }
}
