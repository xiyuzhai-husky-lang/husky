use crate::*;
use husky_token::{IdentifierToken, ScopeResolutionToken, TokenIdx, TokenStream};
use parsec::{ParseContext, ParseFrom};
use thiserror::Error;

pub trait AllocPrincipalEntityPathExpr {
    fn princiapl_entity_path_expr_arena_mut(&mut self) -> &mut PrincipalEntityPathExprArena;
    // fn entity_path_expr_arena_mut(&mut self) -> &mut EntityPathExprArena;
}

#[derive(Debug, PartialEq, Eq)]
pub enum PrincipalEntityPathExpr {
    Root {
        token_idx: TokenIdx,
        ident: Identifier,
        entity_path: EntityPath,
    },
    Subentity {
        parent: PrincipalEntityPathExprIdx,
        scope_resolution_token: ScopeResolutionToken,
        ident_token: IdentifierToken,
    },
}
pub type PrincipalEntityPathExprArena = Arena<PrincipalEntityPathExpr>;
pub type PrincipalEntityPathExprIdx = ArenaIdx<PrincipalEntityPathExpr>;
pub type PrincipalEntityPathExprIdxRange = ArenaIdxRange<PrincipalEntityPathExpr>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PrincipalEntityPathExprError {}

pub type PrincipalEntityPathExprResult<T> = Result<T, PrincipalEntityPathExprError>;

impl PrincipalEntityPathExpr {
    pub(crate) fn parse_from_token_stream<'a>(
        token_stream: &mut TokenStream<'a>,
        princiapl_entity_path_expr_arena: &mut PrincipalEntityPathExprArena,
    ) -> PrincipalEntityPathExprResult<(PrincipalEntityPathExprIdx, EntityPath)> {
        // let ident_token = token_stream.parse_expected::<IdentifierToken>()?;
        todo!()
    }

    pub(crate) fn parse_subentity_from_token_stream<'a>(
        token_stream: &mut TokenStream<'a>,
        princiapl_entity_path_expr_arena: &mut PrincipalEntityPathExprArena,
    ) -> PrincipalEntityPathExprResult<(PrincipalEntityPathExprIdx, EntityPath)> {
        todo!()
    }
}
