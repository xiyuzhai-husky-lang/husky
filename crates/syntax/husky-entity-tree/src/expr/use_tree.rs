use crate::*;
use husky_token::*;
use parsec::{FromAbsent, ParseContext, ParseFrom, StreamWrapper};
use thiserror::Error;

/// use tree expr is top-down
/// because path is resolved top-down
#[derive(Debug, PartialEq, Eq)]
pub enum UseTreeExpr {
    All {
        start: StarToken,
    },
    One {
        ident: IdentifierToken,
    },
    Parent {
        ident: IdentifierToken,
        scope_resolution_token: ScopeResolutionToken,
        children: UseTreeExprChildren,
    },
    Err(UseTreeExprError),
}

#[derive(Debug, PartialEq, Eq)]
pub enum UseTreeExprChildren {
    Single {
        child: UseTreeExprIdx,
    },
    Multiple {
        lcurl: LeftCurlyBraceToken,
        children: UseTreeExprIdxRange,
        commas: Vec<CommaToken>,
        rcurl: RightCurlyBraceToken,
    },
}

pub type UseTreeExprArena = Arena<UseTreeExpr>;
pub type UseTreeExprIdx = ArenaIdx<UseTreeExpr>;
pub type UseTreeExprIdxRange = ArenaIdxRange<UseTreeExpr>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum UseTreeExprError {
    #[error("token error")]
    Token(#[from] TokenError),
}

impl<Context> FromAbsent<IdentifierToken, Context> for UseTreeExprError
where
    Context: ParseContext,
{
    fn new_absent_error(state: Context::State) -> Self {
        todo!()
    }
}

pub type UseExprResult<T> = Result<T, UseTreeExprError>;

pub(crate) struct UseExprParser<'a, 'b> {
    token_stream: TokenStream<'a>,
    use_expr_arena: &'b mut UseTreeExprArena,
    module_symbol_context: ModuleSymbolContext<'a>,
}

impl<'a, 'b> UseExprParser<'a, 'b> {
    pub(crate) fn new(
        token_stream: TokenStream<'a>,
        use_expr_arena: &'b mut UseTreeExprArena,
        module_symbol_context: ModuleSymbolContext<'a>,
    ) -> Self {
        Self {
            token_stream,
            use_expr_arena,
            module_symbol_context,
        }
    }
}

impl<'a, 'b> parsec::HasParseError for UseExprParser<'a, 'b> {
    type Error = UseTreeExprError;
}

impl<'a, 'b> std::ops::Deref for UseExprParser<'a, 'b> {
    type Target = TokenStream<'a>;

    fn deref(&self) -> &Self::Target {
        &self.token_stream
    }
}

impl<'a, 'b> std::ops::DerefMut for UseExprParser<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token_stream
    }
}

impl<'a, 'b> std::borrow::Borrow<TokenStream<'a>> for UseExprParser<'a, 'b> {
    fn borrow(&self) -> &TokenStream<'a> {
        &self.token_stream
    }
}

impl<'a, 'b> std::borrow::BorrowMut<TokenStream<'a>> for UseExprParser<'a, 'b> {
    fn borrow_mut(&mut self) -> &mut TokenStream<'a> {
        &mut self.token_stream
    }
}

impl<'a, 'b> StreamWrapper for UseExprParser<'a, 'b> {}

impl<'a, 'b> UseExprParser<'a, 'b> {
    pub(crate) fn parse_use_expr(&mut self) -> UseExprResult<UseTreeExprIdx> {
        todo!()
        // let ident_token = self.parse_expected::<IdentifierToken>()?;
        // let Some(entity_symbol) = self
        //     .module_symbol_context
        //     .resolve_ident(ident_token.token_idx(),ident_token.ident()) else {
        //         return Err(UseExprError::UnrecognizedIdentifier(ident_token))
        //     };
        // let module_item = match entity_symbol {
        //     EntitySymbol::ModuleItem(module_item) => module_item,
        //     _ => todo!(),
        // };
        // let (expr, path) = if let Some(_) = self.try_parse::<ScopeResolutionToken>() {
        //     todo!()
        // } else {
        //     let path = module_item.path();
        //     (
        //         UseExpr::Root {
        //             ident_token,
        //             entity_path: path.into(),
        //         },
        //         path,
        //     )
        // };
        // let expr = self.princiapl_entity_path_expr_arena.alloc_one(expr);
        // Ok((expr, path))
    }
}
