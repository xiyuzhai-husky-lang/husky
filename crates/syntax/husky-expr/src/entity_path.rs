mod error;

pub use error::*;

use crate::*;
use husky_entity_tree::EntityTreeError;
use parsec::OriginalError;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum EntityPathExpr {
    Root {
        token_idx: TokenIdx,
        ident: Ident,
        entity_path: EntityPath,
    },
    Subentity {
        parent: EntityPathExprIdx,
        scope_resolution_token: ScopeResolutionToken,
        ident_token: EntityPathExprResult<IdentToken>,
        path: EntityPathExprResult<EntityPath>,
    },
}

pub type EntityPathExprArena = Arena<EntityPathExpr>;
pub type EntityPathExprIdx = ArenaIdx<EntityPathExpr>;
