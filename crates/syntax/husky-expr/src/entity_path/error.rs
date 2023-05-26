use super::*;
use original_error::OriginalError;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum EntityPathExprError {
    #[error("original `{0}`")]
    Original(OriginalEntityPathExprError),
    #[error("derived `{0}`")]
    Derived(DerivedEntityPathExprError),
}

impl From<TokenError> for EntityPathExprError {
    fn from(value: TokenError) -> Self {
        EntityPathExprError::Derived(value.into())
    }
}

impl From<OriginalEntityPathExprError> for EntityPathExprError {
    fn from(v: OriginalEntityPathExprError) -> Self {
        Self::Original(v)
    }
}

impl From<DerivedEntityPathExprError> for EntityPathExprError {
    fn from(v: DerivedEntityPathExprError) -> Self {
        Self::Derived(v)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum OriginalEntityPathExprError {
    #[error("entity tree")]
    EntityTree {
        token_idx: TokenIdx,
        error: EntityTreeError,
    },
    #[error("expect identifier after `::`")]
    ExpectIdentAfterScopeResolution(TokenStreamState),
}

impl OriginalError for OriginalEntityPathExprError {
    type Error = EntityPathExprError;
}

impl From<OriginalExprError> for OriginalEntityPathExprError {
    fn from(value: OriginalExprError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum DerivedEntityPathExprError {
    #[error("derived from expr error {0}")]
    AbortFromExprError(#[from] OriginalExprError),
    #[error("token error {0}")]
    TokenError(#[from] TokenError),
}

pub type EntityPathExprResult<T> = Result<T, EntityPathExprError>;
