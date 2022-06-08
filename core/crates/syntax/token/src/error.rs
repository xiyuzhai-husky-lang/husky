use dev_utils::DevSource;
use std::sync::Arc;
use text::TextRange;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LexError {
    pub message: String,
    pub range: TextRange,
    pub dev_src: DevSource,
}

pub type LexResult<T> = Result<T, LexError>;

pub type LexResultArc<T> = Result<Arc<T>, LexError>;
