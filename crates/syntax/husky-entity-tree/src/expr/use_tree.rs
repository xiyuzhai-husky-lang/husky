use crate::*;
use husky_token::*;
use parsec::{FromAbsent, ParseContext, ParseFrom, StreamWrapper};
use thiserror::Error;

/// use tree expr is top-down
/// because path is resolved top-down
#[derive(Debug, PartialEq, Eq)]
pub enum UseTreeExpr {
    All {
        star_token: StarToken,
    },
    One {
        ident_token: IdentifierToken,
    },
    Parent {
        ident_token: IdentifierToken,
        scope_resolution_token: ScopeResolutionToken,
        children: UseTreeExprChildren,
    },
    Err(UseTreeExprError),
}

impl From<UseTreeExprResult<UseTreeExpr>> for UseTreeExpr {
    fn from(value: UseTreeExprResult<UseTreeExpr>) -> Self {
        match value {
            Ok(value) => value,
            Err(e) => UseTreeExpr::Err(e),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum UseTreeExprChildren {
    Single {
        child: UseTreeExprIdx,
    },
    Multiple {
        lcurl_token: LeftCurlyBraceToken,
        children: UseTreeExprIdxRange,
        comma_tokens: Vec<CommaToken>,
        rcurl_token: UseTreeExprResult<RightCurlyBraceToken>,
    },
}

pub type UseTreeExprArena = Arena<UseTreeExpr>;
pub type UseTreeExprIdx = ArenaIdx<UseTreeExpr>;
pub type UseTreeExprIdxRange = ArenaIdxRange<UseTreeExpr>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UseTreeExprRoot {
    use_token: UseToken,
    use_tree_expr_idx: UseTreeExprIdx,
}

pub(crate) fn parse_use_tree_expr_root(
    token_stream: &mut TokenStream,
    use_tree_expr_arena: &mut UseTreeExprArena,
) -> UseTreeExprResult<UseTreeExprRoot> {
    UseTreeExprParser {
        token_stream,
        use_tree_expr_arena,
    }
    .parse_use_expr_root()
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum UseTreeExprError {
    #[error("expect identifier")]
    ExpectIdentifier(TokenIdx),
    #[error("token error")]
    Token(#[from] TokenError),
}

pub type UseTreeExprResult<T> = Result<T, UseTreeExprError>;

impl<'a, 'b> FromAbsent<IdentifierToken, UseTreeExprParser<'a, 'b>> for UseTreeExprError {
    fn new_absent_error(state: TokenIdx) -> Self {
        UseTreeExprError::ExpectIdentifier(state)
    }
}

impl<'a, 'b> FromAbsent<RightCurlyBraceToken, UseTreeExprParser<'a, 'b>> for UseTreeExprError {
    fn new_absent_error(state: TokenIdx) -> Self {
        todo!()
    }
}

impl<'a, 'b> FromAbsent<UseToken, UseTreeExprParser<'a, 'b>> for UseTreeExprError {
    fn new_absent_error(state: TokenIdx) -> Self {
        todo!()
    }
}

impl<'a, 'b> FromAbsent<UseTreeExpr, UseTreeExprParser<'a, 'b>> for UseTreeExprError {
    fn new_absent_error(state: TokenIdx) -> Self {
        todo!()
    }
}

pub type UseExprResult<T> = Result<T, UseTreeExprError>;

pub(crate) struct UseTreeExprParser<'a, 'b> {
    token_stream: &'b mut TokenStream<'a>,
    use_tree_expr_arena: &'b mut UseTreeExprArena,
}

impl<'a, 'b> UseTreeExprParser<'a, 'b> {
    pub(crate) fn new(
        token_stream: &'b mut TokenStream<'a>,
        use_expr_arena: &'b mut UseTreeExprArena,
    ) -> Self {
        Self {
            token_stream,
            use_tree_expr_arena: use_expr_arena,
        }
    }
}

impl<'a, 'b> parsec::HasParseError for UseTreeExprParser<'a, 'b> {
    type Error = UseTreeExprError;
}

impl<'a, 'b> std::ops::Deref for UseTreeExprParser<'a, 'b> {
    type Target = TokenStream<'a>;

    fn deref(&self) -> &Self::Target {
        &self.token_stream
    }
}

impl<'a, 'b> std::ops::DerefMut for UseTreeExprParser<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token_stream
    }
}

impl<'a, 'b> std::borrow::Borrow<TokenStream<'a>> for UseTreeExprParser<'a, 'b> {
    fn borrow(&self) -> &TokenStream<'a> {
        &self.token_stream
    }
}

impl<'a, 'b> std::borrow::BorrowMut<TokenStream<'a>> for UseTreeExprParser<'a, 'b> {
    fn borrow_mut(&mut self) -> &mut TokenStream<'a> {
        &mut self.token_stream
    }
}

impl<'a, 'b> StreamWrapper for UseTreeExprParser<'a, 'b> {}

impl<'a, 'b> UseTreeExprParser<'a, 'b> {
    pub(crate) fn parse_use_expr_root(&mut self) -> UseExprResult<UseTreeExprRoot> {
        let use_token = self.parse_expected::<UseToken>()?;
        let ident_token = self.parse_expected::<IdentifierToken>()?;
        let use_tree_expr = self.parse_use_expr_after_ident(ident_token)?;
        let use_tree_expr_idx = self.use_tree_expr_arena.alloc_one(use_tree_expr);
        Ok(UseTreeExprRoot {
            use_token,
            use_tree_expr_idx,
        })
    }

    fn parse_use_expr_after_ident(
        &mut self,
        ident_token: IdentifierToken,
    ) -> UseExprResult<UseTreeExpr> {
        let Some(scope_resolution_token) = self.parse::<ScopeResolutionToken>()? else {
            return Ok( UseTreeExpr::One { ident_token })
        };
        let children = self.parse_children()?;
        Ok(UseTreeExpr::Parent {
            ident_token,
            scope_resolution_token,
            children,
        })
    }

    fn parse_children(&mut self) -> UseExprResult<UseTreeExprChildren> {
        let Some(lcurl_token) = self.parse::<LeftCurlyBraceToken>()? else {
            let child = self.parse_expected().into();
            let child = self.use_tree_expr_arena.alloc_one(child);
            return Ok(UseTreeExprChildren::Single { child })
        };
        let (children, comma_tokens) = parsec::parse_separated_list(self)?;
        let children = self.use_tree_expr_arena.alloc_batch(children);
        Ok(UseTreeExprChildren::Multiple {
            lcurl_token,
            children,
            comma_tokens,
            rcurl_token: self.parse_expected(),
        })
    }
}

impl<'a, 'b> ParseFrom<UseTreeExprParser<'a, 'b>> for UseTreeExpr {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut UseTreeExprParser<'a, 'b>,
    ) -> Result<Option<Self>, UseTreeExprError> {
        if let Some(star_token) = ctx.parse::<StarToken>()? {
            return Ok(Some(UseTreeExpr::All { star_token }));
        }
        let Some(ident_token) = ctx.parse::<IdentifierToken>()? else {
            return Ok(None);
        };
        Ok(Some(ctx.parse_use_expr_after_ident(ident_token)?))
    }
}
