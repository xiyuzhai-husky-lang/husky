use crate::*;
use husky_entity_tree::EntityTreeError;
use husky_expr::OriginalExprError;
use husky_token::*;
use husky_vfs::VfsError;
use parsec::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DeclError {
    #[error("{0}")]
    Original(OriginalDeclError),
    #[error("{0}")]
    Derived(DerivedDeclError),
}

impl From<EntityTreeError> for DeclError {
    fn from(value: EntityTreeError) -> Self {
        DeclError::Derived(value.into())
    }
}

impl From<VfsError> for DeclError {
    fn from(value: VfsError) -> Self {
        DeclError::Derived(value.into())
    }
}

impl From<TokenError> for DeclError {
    fn from(value: TokenError) -> Self {
        DeclError::Derived(value.into())
    }
}

impl From<DerivedDeclError> for DeclError {
    fn from(v: DerivedDeclError) -> Self {
        Self::Derived(v)
    }
}

impl From<OriginalDeclError> for DeclError {
    fn from(v: OriginalDeclError) -> Self {
        Self::Original(v)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalDeclError {
    #[error("expect `{{` or `(` or `;`")]
    ExpectLCurlOrLParOrSemicolon(TokenIdx),
}

impl OriginalError for OriginalDeclError {
    type Error = DeclError;
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedDeclError {
    #[error("token error")]
    Token(#[from] TokenError),
    #[error("derived {0}")]
    Vfs(#[from] VfsError),
    #[error("derived {0}")]
    EntityTree(#[from] EntityTreeError),
    #[error("derived {0}")]
    ExprError(#[from] DerivedExprError),
    #[error("unable to parse impl block decl for ty as trai method decl")]
    UnableToParseImplDeclForTyAsTraitMethodDecl,
    #[error("unable to parse impl block decl for ty method decl")]
    UnableToParseImplDeclForTyMethodDecl,
    #[error("impl block error")]
    ImplErr,
}

pub type DeclResult<T> = Result<T, DeclError>;
pub type DeclResultRef<'a, T> = Result<T, &'a DeclError>;

impl<Db: DeclDb + ?Sized> salsa::DebugWithDb<Db> for DeclError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        // ad hoc
        std::fmt::Debug::fmt(&self, f)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = DeclDb)]
pub enum DeclExprError {
    #[error("{0}")]
    Original(#[from] OriginalDeclExprError),
    #[error("{0}")]
    Derived(#[from] DerivedDeclExprError),
}

impl From<TokenError> for DeclExprError {
    fn from(error: TokenError) -> Self {
        DeclExprError::Derived(error.into())
    }
}

pub type DeclExprResult<T> = Result<T, DeclExprError>;
pub type DeclExprResultRef<'a, T> = Result<T, &'a DeclExprError>;

impl From<ExprError> for DeclExprError {
    fn from(error: ExprError) -> Self {
        match error {
            ExprError::Original(error) => DeclExprError::Original(error.into()),
            ExprError::Derived(error) => DeclExprError::Derived(error.into()),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = DeclDb)]
pub enum OriginalDeclExprError {
    #[error("derived {0}")]
    Expr(#[from] OriginalExprError),
    #[error("expect output type")]
    ExpectOutputType(TokenIdx),
    #[error("expect `->`")]
    ExpectCurry(TokenIdx),
    #[error("expect `:` at end of line")]
    ExpectEolColon(TokenIdx),
    #[error("expect `}}`")]
    ExpectRightCurlyBrace(TokenIdx),
    #[error("expect `>` for implicit parameters")]
    ExpectRightAngleBracketForImplicitParameterDeclList {
        langle_token_idx: TokenIdx,
        current_token_idx: TokenIdx,
    },
    #[error("expect parameter declaration list")]
    ExpectParameterDeclList(TokenIdx),
    #[error("expect implicit parameter declaration")]
    ExpectImplicitParameterDecl(TokenIdx),
    #[error("expect `)` in parameter list")]
    ExpectRightParenthesisInParameterList(TokenIdx),
}

impl OriginalError for OriginalDeclExprError {
    type Error = DeclExprError;
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = DeclDb)]
pub enum DerivedDeclExprError {
    #[error("{0}")]
    ExprError(#[from] DerivedExprError),
    #[error("{0}")]
    TokenError(#[from] TokenError),
}
