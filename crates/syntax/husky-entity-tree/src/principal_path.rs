use crate::*;
use husky_token::{IdentifierToken, ScopeResolutionToken, TokenError, TokenIdx, TokenStream};
use parsec::{FromAbsent, ParseContext, ParseFrom, StreamWrapper};
use thiserror::Error;

pub trait AllocPrincipalPathExpr {
    fn princiapl_entity_path_expr_arena_mut(&mut self) -> &mut PrincipalPathExprArena;
    // fn entity_path_expr_arena_mut(&mut self) -> &mut EntityPathExprArena;
}

#[derive(Debug, PartialEq, Eq)]
pub enum PrincipalPathExpr {
    Root {
        ident_token: IdentifierToken,
        entity_path: EntityPath,
    },
    Subentity {
        parent: PrincipalPathExprIdx,
        scope_resolution_token: ScopeResolutionToken,
        ident_token: IdentifierToken,
    },
}
pub type PrincipalPathExprArena = Arena<PrincipalPathExpr>;
pub type PrincipalPathExprIdx = ArenaIdx<PrincipalPathExpr>;
pub type PrincipalPathExprIdxRange = ArenaIdxRange<PrincipalPathExpr>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PrincipalPathExprError {
    #[error("token error")]
    Token(#[from] TokenError),
    #[error("unrecognized identifier")]
    UnrecognizedIdentifier(IdentifierToken),
}

impl<Context> FromAbsent<IdentifierToken, Context> for PrincipalPathExprError
where
    Context: ParseContext,
{
    fn new_absent_error(state: Context::State) -> Self {
        todo!()
    }
}

pub type PrincipalPathExprResult<T> = Result<T, PrincipalPathExprError>;

pub(crate) struct PrincipalPathExprParser<'a, 'b> {
    token_stream: TokenStream<'a>,
    princiapl_entity_path_expr_arena: &'b mut PrincipalPathExprArena,
    module_symbol_context: ModuleSymbolContext<'a>,
}

impl<'a, 'b> PrincipalPathExprParser<'a, 'b> {
    pub(crate) fn new(
        token_stream: TokenStream<'a>,
        princiapl_entity_path_expr_arena: &'b mut PrincipalPathExprArena,
        module_symbol_context: ModuleSymbolContext<'a>,
    ) -> Self {
        Self {
            token_stream,
            princiapl_entity_path_expr_arena,
            module_symbol_context,
        }
    }
}

impl<'a, 'b> parsec::HasParseError for PrincipalPathExprParser<'a, 'b> {
    type Error = PrincipalPathExprError;
}

impl<'a, 'b> std::ops::Deref for PrincipalPathExprParser<'a, 'b> {
    type Target = TokenStream<'a>;

    fn deref(&self) -> &Self::Target {
        &self.token_stream
    }
}

impl<'a, 'b> std::ops::DerefMut for PrincipalPathExprParser<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token_stream
    }
}

impl<'a, 'b> std::borrow::Borrow<TokenStream<'a>> for PrincipalPathExprParser<'a, 'b> {
    fn borrow(&self) -> &TokenStream<'a> {
        &self.token_stream
    }
}

impl<'a, 'b> std::borrow::BorrowMut<TokenStream<'a>> for PrincipalPathExprParser<'a, 'b> {
    fn borrow_mut(&mut self) -> &mut TokenStream<'a> {
        &mut self.token_stream
    }
}

impl<'a, 'b> StreamWrapper for PrincipalPathExprParser<'a, 'b> {}

impl<'a, 'b> PrincipalPathExprParser<'a, 'b> {
    pub(crate) fn parse_principal_entity_path_expr(
        &mut self,
    ) -> PrincipalPathExprResult<(PrincipalPathExprIdx, ModuleItemPath)> {
        let ident_token = self.parse_expected::<IdentifierToken>()?;
        let Some(entity_symbol) = self
            .module_symbol_context
            .resolve_ident(ident_token.token_idx(),ident_token.ident()) else {
                return Err(PrincipalPathExprError::UnrecognizedIdentifier(ident_token))
            };
        let module_item = match entity_symbol {
            EntitySymbol::ModuleItem(module_item) => module_item,
            _ => todo!(),
        };
        let (expr, path) = if let Some(_) = self.try_parse::<ScopeResolutionToken>() {
            todo!()
        } else {
            let path = module_item.path();
            (
                PrincipalPathExpr::Root {
                    ident_token,
                    entity_path: path.into(),
                },
                path,
            )
        };
        let expr = self.princiapl_entity_path_expr_arena.alloc_one(expr);
        Ok((expr, path))
    }
}

// impl PrincipalPathExpr {
//     pub(crate) fn parse_from_token_stream<'a>(
//         token_stream: &mut TokenStream<'a>,
//         princiapl_entity_path_expr_arena: &mut PrincipalPathExprArena,
//     ) -> PrincipalPathExprResult<(PrincipalPathExprIdx, EntityPath)> {
//         let ident_token = token_stream.parse_expected::<IdentifierToken>()?;
//         todo!()
//     }

//     pub(crate) fn parse_subentity_from_token_stream<'a>(
//         token_stream: &mut TokenStream<'a>,
//         princiapl_entity_path_expr_arena: &mut PrincipalPathExprArena,
//     ) -> PrincipalPathExprResult<(PrincipalPathExprIdx, EntityPath)> {
//         todo!()
//     }
// }
