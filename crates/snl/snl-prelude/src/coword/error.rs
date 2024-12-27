#[derive(Debug, thiserror::Error)]
pub enum SnlCowordError {
    #[error("invalid ident: {0}")]
    InvalidIdent(String),
}

pub type SnlCowordResult<T> = Result<T, SnlCowordError>;
