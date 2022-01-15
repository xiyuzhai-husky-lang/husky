use ast::AstError;
use common::*;
use scope::ScopeError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Origin {
    file: file::FileId,
    range: text::TextRange,
}

pub type SemanticResult<T> = Result<T, SemanticError>;

pub type SemanticResultArc<T> = Result<std::sync::Arc<T>, SemanticError>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SemanticError {
    origin: Option<Origin>,
    src: Option<DevSource>,
    kind: SemanticErrorKind,
}

impl From<ScopeError> for SemanticError {
    fn from(error: ScopeError) -> Self {
        Self {
            origin: None,
            src: None,
            kind: SemanticErrorKind::ScopeError(error),
        }
    }
}

impl From<AstError> for SemanticError {
    fn from(_: AstError) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SemanticErrorKind {
    ScopeError(ScopeError),
}
