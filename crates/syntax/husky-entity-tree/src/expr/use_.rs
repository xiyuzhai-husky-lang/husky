use crate::*;
use husky_token::*;
use parsec::{FromAbsent, ParseContext, ParseFrom, StreamWrapper};
use thiserror::Error;

/// use tree expr is top-down
/// because path is resolved top-down
#[derive(Debug, PartialEq, Eq)]
pub enum UseExpr {
    All {
        star_token: StarToken,
    },
    One {
        ident_token: IdentifierToken,
    },
    Parent {
        ident_token: IdentifierToken,
        scope_resolution_token: ScopeResolutionToken,
        children: UseExprChildren,
    },
    Err(UseExprError),
}

impl From<UseExprResult<UseExpr>> for UseExpr {
    fn from(value: UseExprResult<UseExpr>) -> Self {
        match value {
            Ok(value) => value,
            Err(e) => UseExpr::Err(e),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum UseExprChildren {
    Single {
        child: UseExprIdx,
    },
    Multiple {
        lcurl_token: LeftCurlyBraceToken,
        children: UseExprIdxRange,
        comma_tokens: Vec<CommaToken>,
        rcurl_token: UseExprResult<RightCurlyBraceToken>,
    },
}
impl UseExprChildren {
    pub(crate) fn idx_range(&self) -> UseExprIdxRange {
        match self {
            UseExprChildren::Single { child } => UseExprIdxRange::new_single(*child),
            UseExprChildren::Multiple {
                lcurl_token,
                children,
                comma_tokens,
                rcurl_token,
            } => *children,
        }
    }
}

pub type UseExprArena = Arena<UseExpr>;
pub type UseExprIdx = ArenaIdx<UseExpr>;
pub type UseExprIdxRange = ArenaIdxRange<UseExpr>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct UseExprRoot {
    use_token: UseToken,
    use_expr_idx: UseExprIdx,
}

impl UseExprRoot {
    pub fn use_token(&self) -> UseToken {
        self.use_token
    }

    pub fn use_expr_idx(&self) -> ArenaIdx<UseExpr> {
        self.use_expr_idx
    }
}

pub(crate) fn parse_use_expr_root(
    token_stream: &mut TokenStream,
    use_expr_arena: &mut UseExprArena,
) -> UseExprResult<UseExprRoot> {
    UseExprParser {
        token_stream,
        use_expr_arena,
    }
    .parse_use_expr_root()
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum UseExprError {
    #[error("expect identifier")]
    ExpectIdentifier(TokenIdx),
    #[error("token error")]
    Token(#[from] TokenError),
}

impl<'a, 'b> FromAbsent<IdentifierToken, UseExprParser<'a, 'b>> for UseExprError {
    fn new_absent_error(state: TokenIdx) -> Self {
        UseExprError::ExpectIdentifier(state)
    }
}

impl<'a, 'b> FromAbsent<RightCurlyBraceToken, UseExprParser<'a, 'b>> for UseExprError {
    fn new_absent_error(state: TokenIdx) -> Self {
        todo!()
    }
}

impl<'a, 'b> FromAbsent<UseToken, UseExprParser<'a, 'b>> for UseExprError {
    fn new_absent_error(state: TokenIdx) -> Self {
        todo!()
    }
}

impl<'a, 'b> FromAbsent<UseExpr, UseExprParser<'a, 'b>> for UseExprError {
    fn new_absent_error(state: TokenIdx) -> Self {
        todo!()
    }
}

pub type UseExprResult<T> = Result<T, UseExprError>;

pub(crate) struct UseExprParser<'a, 'b> {
    token_stream: &'b mut TokenStream<'a>,
    use_expr_arena: &'b mut UseExprArena,
}

impl<'a, 'b> UseExprParser<'a, 'b> {
    pub(crate) fn new(
        token_stream: &'b mut TokenStream<'a>,
        use_expr_arena: &'b mut UseExprArena,
    ) -> Self {
        Self {
            token_stream,
            use_expr_arena: use_expr_arena,
        }
    }
}

impl<'a, 'b> parsec::HasParseError for UseExprParser<'a, 'b> {
    type Error = UseExprError;
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
    pub(crate) fn parse_use_expr_root(&mut self) -> UseExprResult<UseExprRoot> {
        let use_token = self.parse_expected::<UseToken>()?;
        let ident_token = self.parse_expected::<IdentifierToken>()?;
        let use_expr = self.parse_use_expr_after_ident(ident_token)?;
        let use_expr_idx = self.use_expr_arena.alloc_one(use_expr);
        Ok(UseExprRoot {
            use_token,
            use_expr_idx,
        })
    }

    fn parse_use_expr_after_ident(
        &mut self,
        ident_token: IdentifierToken,
    ) -> UseExprResult<UseExpr> {
        let Some(scope_resolution_token) = self.parse::<ScopeResolutionToken>()? else {
            return Ok( UseExpr::One { ident_token })
        };
        let children = self.parse_children()?;
        Ok(UseExpr::Parent {
            ident_token,
            scope_resolution_token,
            children,
        })
    }

    fn parse_children(&mut self) -> UseExprResult<UseExprChildren> {
        let Some(lcurl_token) = self.parse::<LeftCurlyBraceToken>()? else {
            let child = self.parse_expected().into();
            let child = self.use_expr_arena.alloc_one(child);
            return Ok(UseExprChildren::Single { child })
        };
        let (children, comma_tokens) = parsec::parse_separated_list(self)?;
        let children = self.use_expr_arena.alloc_batch(children);
        Ok(UseExprChildren::Multiple {
            lcurl_token,
            children,
            comma_tokens,
            rcurl_token: self.parse_expected(),
        })
    }
}

impl<'a, 'b> ParseFrom<UseExprParser<'a, 'b>> for UseExpr {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut UseExprParser<'a, 'b>,
    ) -> Result<Option<Self>, UseExprError> {
        if let Some(star_token) = ctx.parse::<StarToken>()? {
            return Ok(Some(UseExpr::All { star_token }));
        }
        let Some(ident_token) = ctx.parse::<IdentifierToken>()? else {
            return Ok(None);
        };
        Ok(Some(ctx.parse_use_expr_after_ident(ident_token)?))
    }
}
