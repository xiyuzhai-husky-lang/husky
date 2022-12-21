use husky_text::TextRange;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum AstError {
    #[error("excessive indent")]
    ExcessiveIndent,
    #[error("standalone elif")]
    StandaloneElif,
    #[error("standalone else")]
    StandaloneElse,
    #[error("expect entity keyword")]
    ExpectEntityKeyword,
    #[error("expect decorator or entity keyword")]
    ExpectDecoratorOrEntityKeyword,
    #[error("expect identifier")]
    ExpectIdentifier(Option<TextRange>),
    #[error("expect `(` or decorator or identifier")]
    ExpectParBraOrDecoratorOrIdentifier(Option<TextRange>),
    #[error("expect nothing")]
    ExpectNothing,
}

impl From<&AstError> for AstError {
    fn from(value: &AstError) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstErrorVariant {
    Original { message: String, range: TextRange },
    Derived,
}

pub type AstResult<T> = Result<T, AstError>;
