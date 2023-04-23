use husky_declarative_signature::DeclarativeSignatureError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EtherealSignatureError {}

impl From<DeclarativeSignatureError> for EtherealSignatureError {
    fn from(value: DeclarativeSignatureError) -> Self {
        todo!()
    }
}

pub type EtherealSignatureResult<T> = Result<T, EtherealSignatureError>;
