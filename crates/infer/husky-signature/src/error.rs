#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum SignatureError {}

pub type SignatureResult<T> = Result<T, SignatureError>;
