use crate::*;
use husky_token::*;
use parsec::*;
use thiserror::Error;

/// major path expr is bottom-up
#[derive(Debug, PartialEq, Eq)]
pub enum MajorPathExpr {
    Root {
        ident_token: IdentifierToken,
        entity_path: EntityPath,
    },
    Subentity {
        parent: MajorPathExprIdx,
        scope_resolution_token: ScopeResolutionToken,
        ident_token: IdentifierToken,
    },
}
pub type MajorPathExprArena = Arena<MajorPathExpr>;
pub type MajorPathExprIdx = ArenaIdx<MajorPathExpr>;
pub type MajorPathExprIdxRange = ArenaIdxRange<MajorPathExpr>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MajorPathExprError {
    #[error("token error")]
    Token(#[from] TokenError),
    #[error("unrecognized identifier")]
    UnrecognizedIdentifier(IdentifierToken),
}

impl<Context> FromAbsent<IdentifierToken, Context> for MajorPathExprError
where
    Context: ParseContext,
{
    fn new_absent_error(state: Context::State) -> Self {
        todo!()
    }
}

pub type MajorPathExprResult<T> = Result<T, MajorPathExprError>;

pub(crate) struct MajorPathExprParser<'a, 'b> {
    token_stream: TokenStream<'a>,
    major_path_expr_arena: &'b mut MajorPathExprArena,
    module_symbol_context: ModuleSymbolContext<'a>,
}

impl<'a, 'b> MajorPathExprParser<'a, 'b> {
    pub(crate) fn new(
        token_stream: TokenStream<'a>,
        major_path_expr_arena: &'b mut MajorPathExprArena,
        module_symbol_context: ModuleSymbolContext<'a>,
    ) -> Self {
        Self {
            token_stream,
            major_path_expr_arena,
            module_symbol_context,
        }
    }
}

impl<'a, 'b> MajorPathExprParser<'a, 'b> {
    pub(crate) fn parse_principal_path_expr(
        &mut self,
    ) -> MajorPathExprResult<(MajorPathExprIdx, ModuleItemPath)> {
        let ident_token = self.parse_expected::<IdentifierToken>()?;
        let Some(entity_symbol) = self
            .module_symbol_context
            .resolve_ident(ident_token.token_idx(),ident_token.ident()) else {
                return Err(MajorPathExprError::UnrecognizedIdentifier(ident_token))
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
                MajorPathExpr::Root {
                    ident_token,
                    entity_path: path.into(),
                },
                path,
            )
        };
        let expr = self.major_path_expr_arena.alloc_one(expr);
        Ok((expr, path))
    }
}

impl<'a, 'b> parsec::HasParseError for MajorPathExprParser<'a, 'b> {
    type Error = MajorPathExprError;
}

impl<'a, 'b> std::ops::Deref for MajorPathExprParser<'a, 'b> {
    type Target = TokenStream<'a>;

    fn deref(&self) -> &Self::Target {
        &self.token_stream
    }
}

impl<'a, 'b> std::ops::DerefMut for MajorPathExprParser<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token_stream
    }
}

impl<'a, 'b> std::borrow::Borrow<TokenStream<'a>> for MajorPathExprParser<'a, 'b> {
    fn borrow(&self) -> &TokenStream<'a> {
        &self.token_stream
    }
}

impl<'a, 'b> std::borrow::BorrowMut<TokenStream<'a>> for MajorPathExprParser<'a, 'b> {
    fn borrow_mut(&mut self) -> &mut TokenStream<'a> {
        &mut self.token_stream
    }
}

impl<'a, 'b> StreamWrapper for MajorPathExprParser<'a, 'b> {}
