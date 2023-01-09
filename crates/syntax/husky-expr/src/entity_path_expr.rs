use crate::*;

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
        ident_token: ExprResult<IdentifierToken>,
    },
}

pub type EntityPathExprArena = Arena<EntityPathExpr>;
pub type EntityPathExprIdx = ArenaIdx<EntityPathExpr>;
