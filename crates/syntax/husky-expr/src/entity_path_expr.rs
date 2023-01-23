use crate::*;
use husky_entity_tree::EntityTreeError;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
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

impl From<ExprError> for OriginalEntityPathExprError {
    fn from(value: ExprError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedEntityPathExprError {
    #[error("derived from expr error {0}")]
    AbortFromExprError(#[from] ExprError),
}

pub type EntityPathExprResult<T> = Result<T, EntityPathExprError>;
