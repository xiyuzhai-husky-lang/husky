use crate::*;
use husky_token::{IdentifierToken, ScopeResolutionToken, TokenIdx};
use parsec::{ParseContext, ParseFrom};
use thiserror::Error;

pub trait AllocPrincipalEntityPathExpr {
    fn princiapl_entity_path_expr_arena_mut(&mut self) -> &mut PrincipalEntityPathExprArena;
    // fn entity_path_expr_arena_mut(&mut self) -> &mut EntityPathExprArena;
}

pub enum PrincipalEntityPathExpr {
    Root {
        token_idx: TokenIdx,
        ident: Identifier,
        entity_path: EntityPath,
    },
    Subentity {
        parent: PrincipalEntityPathExprIdx,
        scope_resolution_token: ScopeResolutionToken,
        ident_token: PrincipalEntityPathExprResult<IdentifierToken>,
    },
}
pub type PrincipalEntityPathExprArena = Arena<PrincipalEntityPathExpr>;
pub type PrincipalEntityPathExprIdx = ArenaIdx<PrincipalEntityPathExpr>;
pub type PrincipalEntityPathExprIdxRange = ArenaIdxRange<PrincipalEntityPathExpr>;

#[derive(Debug, Error)]
pub enum PrincipalEntityPathExprError {}

pub type PrincipalEntityPathExprResult<T> = Result<T, PrincipalEntityPathExprError>;

impl<Context> ParseFrom<Context> for PrincipalEntityPathExpr
where
    Context: ParseContext + ?Sized,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context>::Error> {
        todo!()
    }
}

// pub enum EntityPathExpr {
//     Root {},
//     Subentity,
// }
// pub type EntityPathExprArena = Arena<EntityPathExpr>;
// pub type EntityPathExprIdx = ArenaIdx<EntityPathExpr>;
// pub type EntityPathExprIdxRange = ArenaIdxRange<EntityPathExpr>;
