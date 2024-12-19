use snl_prelude::coword::error::SnlCowordError;

#[derive(Debug, thiserror::Error)]
pub enum SnlGrammarError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    // TODO: be more informative, refine it
    #[error("invalid ident: {0}")]
    Coword(#[from] SnlCowordError),
}

pub type SnlGrammarResult<T> = Result<T, SnlGrammarError>;
