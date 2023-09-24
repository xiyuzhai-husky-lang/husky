use super::*;
use original_error::OriginalError;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum PrincipalEntityPathExprError {
    #[error("original `{0}`")]
    Original(OriginalSynExprError),
    #[error("derived `{0}`")]
    Derived(DerivedPrincipalEntityPathExprError),
}

impl From<TokenDataError> for PrincipalEntityPathExprError {
    fn from(value: TokenDataError) -> Self {
        PrincipalEntityPathExprError::Derived(value.into())
    }
}

impl From<OriginalSynExprError> for PrincipalEntityPathExprError {
    fn from(v: OriginalSynExprError) -> Self {
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
pub enum OriginalSynExprError {
    #[error("item tree")]
    EntityTree {
        regional_token_idx: RegionalTokenIdx,
        error: EntitySynTreeError,
    },
    #[error("expect identifier after `::`")]
    ExpectIdentAfterScopeResolution(RegionalTokenStreamState),
}

impl OriginalError for OriginalSynExprError {
    type Error = PrincipalEntityPathExprError;
}

impl From<OriginalSynExprError> for OriginalSynExprError {
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
    TokenDataError(#[from] TokenDataError),
}

pub type SynExprResult<T> = Result<T, PrincipalEntityPathExprError>;
