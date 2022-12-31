use crate::*;
use husky_entity_tree::EntityTreeError;
use husky_expr::ExprError;
use husky_token::*;
use husky_vfs::VfsError;
use parsec::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum DeclError {
    #[error("expect `{{` or `(` or `;`")]
    ExpectLCurlOrLParOrSemicolon(TokenIdx),
    #[error("token error")]
    Token(#[from] TokenError),
    #[error("derived {0}")]
    EntityTree(#[from] EntityTreeError),
    #[error("derived {0}")]
    Vfs(#[from] VfsError),
    #[error("derived {0}")]
    Expr(#[from] ExprError),
}

pub type DeclResult<T> = Result<T, DeclError>;
