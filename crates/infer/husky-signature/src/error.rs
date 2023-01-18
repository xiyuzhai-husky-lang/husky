pub enum SignatureError {}

pub type SignatureResult<T> = Result<T, SignatureError>;
