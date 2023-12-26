use super::*;
use original_error::OriginalError;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db]
pub enum SynExprError {
    #[error("original `{0}`")]
    Original(OriginalSynExprError),
    #[error("derived `{0}`")]
    Derived(DerivedPrincipalEntityPathExprError),
}

impl From<TokenDataError> for SynExprError {
    fn from(value: TokenDataError) -> Self {
        SynExprError::Derived(value.into())
    }
}

impl From<OriginalSynExprError> for SynExprError {
    fn from(v: OriginalSynExprError) -> Self {
        Self::Original(v)
    }
}

impl From<DerivedPrincipalEntityPathExprError> for SynExprError {
    fn from(v: DerivedPrincipalEntityPathExprError) -> Self {
        Self::Derived(v)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db]
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
    type Error = SynExprError;
}

impl From<OriginalSynExprError> for OriginalSynExprError {
    fn from(value: OriginalSynExprError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db]
pub enum DerivedPrincipalEntityPathExprError {
    #[error("derived from expr error {0}")]
    AbortFromExprError(#[from] OriginalSynExprError),
    #[error("token error {0}")]
    TokenDataError(#[from] TokenDataError),
}

pub type SynExprResult<T> = Result<T, SynExprError>;
