use crate::*;
use husky_entity_tree::EntityTreeError;
use outcome::Outcome;
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
        path: EntityPathExprOutcome<EntityPath>,
    },
}

pub type EntityPathExprArena = Arena<EntityPathExpr>;
pub type EntityPathExprIdx = ArenaIdx<EntityPathExpr>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EntityPathExprError {
    #[error("entity tree")]
    EntityTree {
        token_idx: TokenIdx,
        error: EntityTreeError,
    },
    #[error("expect identifier after `::`")]
    ExpectIdentifierAfterScopeResolution(TokenIdx),
}

impl From<ExprError> for EntityPathExprError {
    fn from(value: ExprError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EntityPathExprAbortion {
    #[error("derived from expr error {0}")]
    AbortFromExprError(#[from] ExprError),
}

pub type EntityPathExprResult<T> = Result<T, EntityPathExprError>;
pub type EntityPathExprOutcome<T> = Outcome<T, EntityPathExprError, EntityPathExprAbortion>;
