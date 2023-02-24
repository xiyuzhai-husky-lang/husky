use crate::*;
use husky_entity_tree::EntityTreeError;
use parsec::OriginalError;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum EntityPathExpr {
    Root {
        token_idx: TokenIdx,
        ident: Identifier,
        entity_path: EntityPath,
    },
    Subentity {
        parent: EntityPathExprIdx,
        scope_resolution_token: ScopeResolutionToken,
        ident_token: EntityPathExprResult<IdentifierToken>,
        path: EntityPathExprResult<EntityPath>,
    },
}

pub type EntityPathExprArena = Arena<EntityPathExpr>;
pub type EntityPathExprIdx = ArenaIdx<EntityPathExpr>;

#[derive(Debug, Error, PartialEq, Eq)]
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
pub enum OriginalEntityPathExprError {
    #[error("entity tree")]
    EntityTree {
        token_idx: TokenIdx,
        error: EntityTreeError,
    },
    #[error("expect identifier after `::`")]
    ExpectIdentifierAfterScopeResolution(TokenIdx),
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
pub enum DerivedEntityPathExprError {
    #[error("derived from expr error {0}")]
    AbortFromExprError(#[from] OriginalExprError),
    #[error("token error {0}")]
    TokenError(#[from] TokenError),
}

pub type EntityPathExprResult<T> = Result<T, EntityPathExprError>;
