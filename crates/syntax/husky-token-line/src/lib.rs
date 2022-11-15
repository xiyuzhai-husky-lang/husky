use husky_text::TextIndent;
use husky_token_storage::TokenRange;

pub struct TokenGroup {
    pub indent: TextIndent,
    pub tokens: TokenRange,
}
