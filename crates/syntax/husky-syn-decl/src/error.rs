use crate::*;

use husky_regional_token::{RegionalTokenIdx, RegionalTokenStreamState};
use husky_token_data::TokenDataError;

use original_error::OriginalError;

use thiserror::Error;

#[salsa::derive_debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SynNodeDeclError {
    #[error("{0}")]
    Original(#[from] OriginalSynNodeDeclError),
    #[error("{0}")]
    Derived(#[from] DerivedSynNodeDeclError),
}

pub type SynNodeDeclResult<T> = Result<T, SynNodeDeclError>;
pub type SynNodeDeclErrorRefs<'a> = smallvec::SmallVec<[&'a SynNodeDeclError; 4]>;

impl From<TokenDataError> for SynNodeDeclError {
    fn from(error: TokenDataError) -> Self {
        SynNodeDeclError::Derived(error.into())
    }
}

impl From<SynExprError> for DeclError {
    fn from(_value: SynExprError) -> Self {
        todo!()
    }
}

impl From<SynExprError> for SynNodeDeclError {
    fn from(error: SynExprError) -> Self {
        match error {
            SynExprError::Original(error) => SynNodeDeclError::Original(error.into()),
            SynExprError::Derived(error) => SynNodeDeclError::Derived(error.into()),
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalSynNodeDeclError {
    #[error("derived {0}")]
    Expr(#[from] OriginalSynExprError),
    #[error("expect output type")]
    ExpectedOutputType(RegionalTokenStreamState),
    #[error("expect `->`")]
    ExpectedCurry(RegionalTokenStreamState),
    #[error("expect `:` at end of line")]
    ExpectedEolColon(RegionalTokenStreamState),
    #[error("expected `}}` for struct")]
    ExpectedRcurlForStruct(RegionalTokenStreamState),
    #[error("expected `}}` for type props variant")]
    ExpectedRcurlForTypePropsVariant(RegionalTokenStreamState),
    #[error("expect `>` for implicit parameters")]
    ExpectedRightAngleDelimiterForImplicitParameterDeclList {
        langle_regional_token_idx: RegionalTokenIdx,
        regional_token_stream_state: RegionalTokenStreamState,
    },
    #[error("expect parameter declaration list")]
    ExpectedParameterDeclList(RegionalTokenStreamState),
    #[error("expect implicit parameter declaration")]
    ExpectedImplicitParameterDecl(RegionalTokenStreamState),
    #[error("expect `)` in parameter list")]
    ExpectedRightParenthesisInParameterList(RegionalTokenStreamState),
    #[error("expect `)` in parameter list")]
    ExpectedRightParenthesisInTupleStructFieldTypeList(RegionalTokenStreamState),
    #[error("ExpectedColonBeforeValReturnType")]
    ExpectedColonBeforeValReturnType(RegionalTokenStreamState),
    #[error("ExpectVariableType")]
    ExpectedValReturnType(RegionalTokenStreamState),
    #[error("ExpectEqTokenForVariable")]
    ExpectEqTokenForVariable(RegionalTokenStreamState),
    #[error("expected `{{` `(` or `;` for struct")]
    ExpectedLcurlOrLparOrSemicolonForStruct(RegionalTokenStreamState),
    #[error("expected `=` for associated type")]
    ExpectedEqForAssocType(RegionalTokenStreamState),
    #[error("expected `(` for derive")]
    ExpectLeftDelimiterInDerive(RegionalTokenStreamState),
    #[error("expected `)` for derive")]
    ExpectRightDelimiterInDerive(RegionalTokenStreamState),
}

impl OriginalError for OriginalSynNodeDeclError {
    type Error = SynNodeDeclError;
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedSynNodeDeclError {
    #[error("{0}")]
    ExprError(#[from] DerivedSynExprError),
    #[error("{0}")]
    TokenDataError(#[from] TokenDataError),
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db]
pub enum DeclError {
    #[error("declaration expression error")]
    NodeDecl,
    // #[error("{0}")]
    // Original(#[from] OriginalDeclError),
    // #[error("{0}")]
    // Derived(#[from] DerivedDeclError),
}

pub type DeclResult<T> = Result<T, DeclError>;

impl From<&SynNodeDeclError> for DeclError {
    #[track_caller]
    fn from(_value: &SynNodeDeclError) -> Self {
        todo!("not yet expect errors in from of impl From<&SynNodeDeclError> for DeclError");
        // DeclError::NodeDecl
    }
}
