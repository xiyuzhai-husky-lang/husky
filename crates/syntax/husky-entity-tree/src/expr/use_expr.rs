use crate::*;
use husky_token::*;
use parsec::{OriginalError, ParseContext, ParseFrom, StreamWrapper};
use thiserror::Error;

/// use tree expr is top-down
/// because path is resolved top-down
#[derive(Debug, PartialEq, Eq)]
pub enum UseExpr {
    All {
        star_token: StarToken,
    },
    Leaf {
        ident_token: IdentifierToken,
    },
    SelfOne {
        self_token: SelfValueToken,
    },
    Parent {
        parent_name_token: ParentNameToken,
        scope_resolution_token: UseExprResult<ScopeResolutionToken>,
        children: UseExprResult<UseExprChildren>,
    },
    Err(UseExprError),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParentNameToken {
    Identifier(IdentifierToken),
    Crate(CrateToken),
    SelfValue(SelfValueToken),
    Super(SuperToken),
}
impl ParentNameToken {
    pub fn token_idx(&self) -> TokenIdx {
        match self {
            ParentNameToken::Identifier(token) => token.token_idx(),
            ParentNameToken::Crate(token) => token.token_idx(),
            ParentNameToken::SelfValue(token) => token.token_idx(),
            ParentNameToken::Super(_) => todo!(),
        }
    }
    pub fn ident(&self) -> Option<Identifier> {
        match self {
            ParentNameToken::Identifier(ident_token) => Some(ident_token.ident()),
            ParentNameToken::Crate(_)
            | ParentNameToken::SelfValue(_)
            | ParentNameToken::Super(_) => None,
        }
    }
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
    #[error("{0}")]
    Original(#[from] OriginalUseExprError),
    #[error("{0}")]
    Derived(#[from] DerivedUseExprError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalUseExprError {
    #[error("expect identifier")]
    ExpectIdentifier(TokenIdx),
    #[error("expect use expr")]
    ExpectUseExpr(TokenIdx),
    #[error("expect `::`")]
    ExpectScopeResolution(TokenIdx),
    #[error("expect `}}`")]
    ExpectRightCurlyBrace(TokenIdx),
    #[error("expect `use` token")]
    ExpectUseToken(TokenIdx),
}

impl OriginalError for OriginalUseExprError {
    type Error = UseExprError;
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedUseExprError {
    #[error("token error")]
    Token(#[from] TokenError),
}

impl From<TokenError> for UseExprError {
    fn from(value: TokenError) -> Self {
        UseExprError::Derived(value.into())
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
        let use_token: UseToken = self.parse_expected(OriginalUseExprError::ExpectUseToken)?;
        let use_expr = self.parse_expected(OriginalUseExprError::ExpectUseExpr)?;
        Ok(UseExprRoot {
            use_token,
            use_expr_idx: self.use_expr_arena.alloc_one(use_expr),
        })
    }

    fn parse_use_expr_after_ident(
        &mut self,
        ident_token: IdentifierToken,
    ) -> UseExprResult<UseExpr> {
        let Some(scope_resolution_token) = self.parse::<ScopeResolutionToken>()? else {
            return Ok( UseExpr::Leaf { ident_token })
        };
        Ok(UseExpr::Parent {
            parent_name_token: ParentNameToken::Identifier(ident_token),
            scope_resolution_token: Ok(scope_resolution_token),
            children: self.parse_children(),
        })
    }

    fn parse_children(&mut self) -> UseExprResult<UseExprChildren> {
        let Some(lcurl_token) = self.parse::<LeftCurlyBraceToken>()? else {
            let child = self.parse_expected(OriginalUseExprError::ExpectUseExpr).into();
            let child = self.use_expr_arena.alloc_one(child);
            return Ok(UseExprChildren::Single { child })
        };
        let (children, comma_tokens, result) = parsec::parse_separated_list(self);
        if let Err(e) = result {
            todo!()
        }
        let children = self.use_expr_arena.alloc_batch(children);
        Ok(UseExprChildren::Multiple {
            lcurl_token,
            children,
            comma_tokens,
            rcurl_token: self.parse_expected(OriginalUseExprError::ExpectRightCurlyBrace),
        })
    }
}

impl<'a, 'b> ParseFrom<UseExprParser<'a, 'b>> for UseExpr {
    type Error = UseExprError;
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut UseExprParser<'a, 'b>,
    ) -> UseExprResult<Option<Self>> {
        if let Some(star_token) = ctx.parse::<StarToken>()? {
            return Ok(Some(UseExpr::All { star_token }));
        }
        if let Some(crate_token) = ctx.parse::<CrateToken>()? {
            return Ok(Some(UseExpr::Parent {
                parent_name_token: ParentNameToken::Crate(crate_token),
                scope_resolution_token: ctx
                    .parse_expected(OriginalUseExprError::ExpectScopeResolution),
                children: ctx.parse_children(),
            }));
        }
        if let Some(self_value_token) = ctx.parse::<SelfValueToken>()? {
            // differentiate betwee self one and self children
            todo!()
        }
        let Some(ident_token) = ctx.parse::<IdentifierToken>()? else {
            return Ok(None);
        };
        Ok(Some(ctx.parse_use_expr_after_ident(ident_token)?))
    }
}
