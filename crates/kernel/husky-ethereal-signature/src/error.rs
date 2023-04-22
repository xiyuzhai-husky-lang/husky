#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EtherealSignatureError {}

pub type EtherealSignatureResult<T> = Result<T, EtherealSignatureError>;
