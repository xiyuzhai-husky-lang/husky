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
    #[error("expected output type")]
    ExpectedOutputType(RegionalTokenStreamState),
    #[error("expected `->`")]
    ExpectedCurry(RegionalTokenStreamState),
    #[error("expected `:` at end of line")]
    ExpectedEolColon(RegionalTokenStreamState),
    #[error("expected `}}` for struct")]
    ExpectedRcurlForStruct(RegionalTokenStreamState),
    #[error("expected `}}` for type props variant")]
    ExpectedRcurlForTypePropsVariant(RegionalTokenStreamState),
    #[error("expected `>` for implicit parameters")]
    ExpectedRightAngleDelimiterForImplicitParameterDeclList {
        langle_regional_token_idx: RegionalTokenIdx,
        regional_token_stream_state: RegionalTokenStreamState,
    },
    #[error("expected parameter declaration list")]
    ExpectedParameterDeclList(RegionalTokenStreamState),
    #[error("expected implicit parameter declaration")]
    ExpectedImplicitParameterDecl(RegionalTokenStreamState),
    #[error("expected `)` in parameter list")]
    ExpectedRightParenthesisInParameterList(RegionalTokenStreamState),
    #[error("expected `)` in parameter list")]
    ExpectedRightParenthesisInTupleStructFieldTypeList(RegionalTokenStreamState),
    #[error("expected `:` before `val` return type")]
    ExpectedColonBeforeValReturnType(RegionalTokenStreamState),
    #[error("expect `val` return type")]
    ExpectedValReturnType(RegionalTokenStreamState),
    #[error("expect `:` before `static` return type")]
    ExpectedColonBeforeStaticReturnType(RegionalTokenStreamState),
    #[error("expect `static` return type")]
    ExpectedStaticReturnType(RegionalTokenStreamState),
    #[error("expect `=` for memo")]
    ExpectedEqTokenForMemo(RegionalTokenStreamState),
    #[error("expect `=` for type alias")]
    ExpectedEqTokenForTypeAlias(RegionalTokenStreamState),
    #[error("expect `=` for static")]
    ExpectedEqTokenForStatic(RegionalTokenStreamState),
    #[error("expected `{{` `(` or `;` for struct")]
    ExpectedLcurlOrLparOrSemicolonForStruct(RegionalTokenStreamState),
    #[error("expected `=` for associated type")]
    ExpectedEqForAssocType(RegionalTokenStreamState),
    #[error("expected `(` for derive")]
    ExpectedLeftDelimiterInDerive(RegionalTokenStreamState),
    #[error("expected `)` for derive")]
    ExpectedRightDelimiterInDerive(RegionalTokenStreamState),
    #[error("expect `=` for backprop argument")]
    ExpectedEqTokenForBackpropArgument(RegionalTokenStreamState),
    #[error("expected expression for backprop argument")]
    ExpectedExprForBackpropArgument(RegionalTokenStreamState),
    #[error("expected `:` for trait memoized field")]
    ExpectedColonForTraitMemoizedField(RegionalTokenStreamState),
    #[error("expected lib crate declaration narrative")]
    ExpectedCrateDeclNarrative(RegionalTokenStreamState),
    #[error("expected `=` for lib crate declaration default const excludes")]
    ExpectedEqTokenForLibCrateDefaultConstExcludes(RegionalTokenStreamState),
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
pub enum SynDeclError {
    #[error("declaration expression error")]
    NodeDecl,
}

pub type SynDeclResult<T> = Result<T, SynDeclError>;

impl From<&SynNodeDeclError> for SynDeclError {
    #[track_caller]
    fn from(_value: &SynNodeDeclError) -> Self {
        todo!();
        SynDeclError::NodeDecl
    }
}
