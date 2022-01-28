use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticError {}

pub type SemanticResult<T> = Result<T, SemanticError>;

pub type SemanticResultArc<T> = Result<Arc<T>, SemanticError>;

impl From<ScopeError> for SemanticError {
    fn from(error: ScopeError) -> Self {
        Self {}
    }
}

impl From<&ast::AstError> for SemanticError {
    fn from(_: &ast::AstError) -> Self {
        SemanticError {}
    }
}

macro_rules! err {
    () => {{
        Err(SemanticError {})?
    }};
}
pub(crate) use err;

macro_rules! not_none {
    ($option:expr) => {{
        $option.ok_or(SemanticError {})?
    }};
}
pub(crate) use not_none;
use scope_query::ScopeError;
