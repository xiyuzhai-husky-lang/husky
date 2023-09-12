use super::*;
use original_error::OriginalError;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum PrincipalEntityPathExprError {
    #[error("original `{0}`")]
    Original(OriginalPrincipalEntityPathExprError),
    #[error("derived `{0}`")]
    Derived(DerivedPrincipalEntityPathExprError),
}

impl From<TokenError> for PrincipalEntityPathExprError {
    fn from(value: TokenError) -> Self {
        PrincipalEntityPathExprError::Derived(value.into())
    }
}

impl From<OriginalPrincipalEntityPathExprError> for PrincipalEntityPathExprError {
    fn from(v: OriginalPrincipalEntityPathExprError) -> Self {
        Self::Original(v)
    }
}

impl From<DerivedPrincipalEntityPathExprError> for PrincipalEntityPathExprError {
    fn from(v: DerivedPrincipalEntityPathExprError) -> Self {
        Self::Derived(v)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum OriginalPrincipalEntityPathExprError {
    #[error("item tree")]
    EntityTree {
        token_idx: RegionalTokenIdx,
        error: EntitySynTreeError,
    },
    #[error("expect identifier after `::`")]
    ExpectIdentAfterScopeResolution(RegionalTokenStreamState),
}

impl OriginalError for OriginalPrincipalEntityPathExprError {
    type Error = PrincipalEntityPathExprError;
}

impl From<OriginalSynExprError> for OriginalPrincipalEntityPathExprError {
    fn from(value: OriginalSynExprError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum DerivedPrincipalEntityPathExprError {
    #[error("derived from expr error {0}")]
    AbortFromExprError(#[from] OriginalSynExprError),
    #[error("token error {0}")]
    TokenError(#[from] TokenError),
}

pub type PrincipalEntityPathExprResult<T> = Result<T, PrincipalEntityPathExprError>;
