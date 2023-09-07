use crate::*;
use husky_entity_syn_tree::EntitySynTreeError;
use husky_print_utils::p;
use husky_syn_expr::OriginalExprError;
use husky_token::*;
use husky_vfs::VfsError;
use original_error::IntoError;
use parsec::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynDeclDb)]
pub enum SynNodeDeclError {
    #[error("{0}")]
    Original(#[from] OriginalSynNodeDeclError),
    #[error("{0}")]
    Derived(#[from] DerivedSynNodeDeclError),
}

pub type SynNodeDeclResult<T> = Result<T, SynNodeDeclError>;
pub type SynNodeDeclErrorRefs<'a> = smallvec::SmallVec<[&'a SynNodeDeclError; 4]>;

impl From<TokenError> for SynNodeDeclError {
    fn from(error: TokenError) -> Self {
        SynNodeDeclError::Derived(error.into())
    }
}

impl From<ExprError> for DeclError {
    fn from(value: ExprError) -> Self {
        todo!()
    }
}

impl From<ExprError> for SynNodeDeclError {
    fn from(error: ExprError) -> Self {
        match error {
            ExprError::Original(error) => SynNodeDeclError::Original(error.into()),
            ExprError::Derived(error) => SynNodeDeclError::Derived(error.into()),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynDeclDb)]
pub enum OriginalSynNodeDeclError {
    #[error("derived {0}")]
    Expr(#[from] OriginalExprError),
    #[error("expect output type")]
    ExpectedOutputType(TokenStreamState),
    #[error("expect `->`")]
    ExpectedCurry(TokenStreamState),
    #[error("expect `:` at end of line")]
    ExpectedEolColon(TokenStreamState),
    #[error("expect `}}`")]
    ExpectedRightCurlyBrace(TokenStreamState),
    #[error("expect `>` for implicit parameters")]
    ExpectedRightAngleBracketForImplicitParameterDeclList {
        langle_token_idx: TokenIdx,
        token_stream_state: TokenStreamState,
    },
    #[error("expect parameter declaration list")]
    ExpectedParameterDeclList(TokenStreamState),
    #[error("expect implicit parameter declaration")]
    ExpectedImplicitParameterDecl(TokenStreamState),
    #[error("expect `)` in parameter list")]
    ExpectedRightParenthesisInParameterList(TokenStreamState),
    #[error("expect `)` in parameter list")]
    ExpectedRightParenthesisInTupleStructFieldTypeList(TokenStreamState),
    #[error("ExpectVariableType")]
    ExpectedVariableType(TokenStreamState),
    #[error("ExpectEqTokenForVariable")]
    ExpectEqTokenForVariable(TokenStreamState),
    #[error("expected `{{` `(` or `;` for struct")]
    ExpectedLeftCurlyBraceOrLeftParenthesisOrSemicolonForStruct(TokenStreamState),
    #[error("expected `=` for associated type")]
    ExpectedEqForAssociatedType(TokenStreamState),
    #[error("expected `(` for derive")]
    ExpectLeftBracketInDerive(TokenStreamState),
    #[error("expected `)` for derive")]
    ExpectRightBracketInDerive(TokenStreamState),
}

impl IntoError for OriginalSynNodeDeclError {
    type Error = SynNodeDeclError;
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynDeclDb)]
pub enum DerivedSynNodeDeclError {
    #[error("{0}")]
    ExprError(#[from] DerivedExprError),
    #[error("{0}")]
    TokenError(#[from] TokenError),
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynDeclDb)]
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
    fn from(value: &SynNodeDeclError) -> Self {
        todo!();
        DeclError::NodeDecl
    }
}
