use husky_dev_utils::DevSource;
use husky_text::TextRange;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TokenError {
    pub message: String,
    pub range: TextRange,
    pub dev_src: DevSource,
}

pub type TokenResult<T> = Result<T, TokenError>;
