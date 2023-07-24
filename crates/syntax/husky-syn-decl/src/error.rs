use crate::*;
use husky_item_tree::EntityTreeError;
use husky_syn_expr::OriginalExprError;
use husky_token::*;
use husky_vfs::VfsError;
use original_error::IntoError;
use parsec::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = SynDeclDb)]
pub enum NodeDeclError {
    #[error("{0}")]
    Original(#[from] OriginalNodeDeclError),
    #[error("{0}")]
    Derived(#[from] DerivedNodeDeclError),
}

pub type NodeDeclResult<T> = Result<T, NodeDeclError>;
pub type NodeDeclErrorRefs<'a> = smallvec::SmallVec<[&'a NodeDeclError; 4]>;

impl From<TokenError> for NodeDeclError {
    fn from(error: TokenError) -> Self {
        NodeDeclError::Derived(error.into())
    }
}

impl From<ExprError> for DeclError {
    fn from(value: ExprError) -> Self {
        todo!()
    }
}

impl From<ExprError> for NodeDeclError {
    fn from(error: ExprError) -> Self {
        match error {
            ExprError::Original(error) => NodeDeclError::Original(error.into()),
            ExprError::Derived(error) => NodeDeclError::Derived(error.into()),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = SynDeclDb)]
pub enum OriginalNodeDeclError {
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
}

impl IntoError for OriginalNodeDeclError {
    type Error = NodeDeclError;
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = SynDeclDb)]
pub enum DerivedNodeDeclError {
    #[error("{0}")]
    ExprError(#[from] DerivedExprError),
    #[error("{0}")]
    TokenError(#[from] TokenError),
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = SynDeclDb)]
pub enum DeclError {
    #[error("declaration expression error")]
    Expr,
    // #[error("{0}")]
    // Original(#[from] OriginalDeclError),
    // #[error("{0}")]
    // Derived(#[from] DerivedDeclError),
}

pub type DeclResult<T> = Result<T, DeclError>;

impl From<&NodeDeclError> for DeclError {
    fn from(value: &NodeDeclError) -> Self {
        DeclError::Expr
    }
}
