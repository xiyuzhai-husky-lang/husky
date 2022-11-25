use husky_text::TextIndent;
use husky_token_storage::TokenIdxRange;

pub struct TokenLine {
    pub indent: TextIndent,
    pub tokens: TokenIdxRange,
}
