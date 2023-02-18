use crate::*;
use husky_entity_tree::EntityTreeError;
use husky_expr::OriginalExprError;
use husky_token::*;
use husky_vfs::VfsError;
use parsec::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
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
    Expr(#[from] OriginalExprError),
    #[error("impl block error")]
    ImplBlockErr,
    #[error("missing output type")]
    MissingOutputType(TokenIdx),
    #[error("missing `->`")]
    MissingCurry(TokenIdx),
    #[error("missing `:` at end of line")]
    MissingEolColon(TokenIdx),
    #[error("unable to parse impl block decl for ty as trai method decl")]
    UnableToParseImplBlockDeclForTyAsTraitMethodDecl,
    #[error("unable to parse impl block decl for ty method decl")]
    UnableToParseImplBlockDeclForTyMethodDecl,
}

pub type DeclResult<T> = Result<T, DeclError>;
pub type DeclResultBorrowed<'a, T> = Result<T, &'a DeclError>;

impl<Db: DeclDb + ?Sized> salsa::DebugWithDb<Db> for DeclError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        // ad hoc
        std::fmt::Debug::fmt(&self, f)
    }
}
