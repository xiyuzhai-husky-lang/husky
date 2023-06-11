use super::*;
use husky_opn_syntax::BinaryOpr;
use husky_token::*;
use original_error::OriginalError;
use parsec::{ParseFromStream, StreamParser, StreamWrapper};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct ParentUseExpr {
    pub parent_name_token: PathNameToken,
    pub scope_resolution_token: UseExprResult<ScopeResolutionToken>,
    pub children: UseExprResult<UseExprChildren>,
}

/// use tree expr is top-down
/// because path is resolved top-down
#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum UseExpr {
    All { star_token: StarToken },
    Leaf { ident_token: IdentToken },
    SelfOne { self_mod_token: SelfModToken },
    Parent(ParentUseExpr),
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
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
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
                lcurl_token: _,
                children,
                comma_tokens: _,
                rcurl_token: _,
            } => *children,
        }
    }
}

pub type UseExprArena = Arena<UseExpr>;
pub type UseExprIdx = ArenaIdx<UseExpr>;
pub type UseExprIdxRange = ArenaIdxRange<UseExpr>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct UseExprRoot {
    use_token: UseToken,
    parent_use_expr_idx: ParentUseExprIdx,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct ParentUseExprIdx(UseExprIdx);

impl ParentUseExprIdx {
    pub fn use_expr_idx(self) -> UseExprIdx {
        self.0
    }

    pub(crate) fn index(self, arena: &UseExprArena) -> &ParentUseExpr {
        match arena[self.0] {
            UseExpr::Parent(ref expr) => expr,
            _ => unreachable!(),
        }
    }
}

impl From<ParentUseExprIdx> for UseExprIdx {
    fn from(value: ParentUseExprIdx) -> Self {
        value.0
    }
}

impl UseExprRoot {
    pub fn use_token(&self) -> UseToken {
        self.use_token
    }

    pub fn parent_use_expr_idx(&self) -> ParentUseExprIdx {
        self.parent_use_expr_idx
    }
}

pub(crate) fn parse_use_expr_root(
    token_stream: &mut TokenStream,
    use_expr_arena: &mut UseExprArena,
) -> Result<UseExprRoot, ()> {
    UseExprParser {
        token_stream,
        use_expr_arena,
    }
    .parse_use_expr_root()
    .map_err(|e| {
        // alloc into an use expression, so that the error can be access later for diagnostics
        use_expr_arena.alloc_one(UseExpr::Err(e));
    })
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum UseExprError {
    #[error("{0}")]
    Original(#[from] OriginalUseExprError),
    #[error("{0}")]
    Derived(#[from] DerivedUseExprError),
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum OriginalUseExprError {
    #[error("expect identifier")]
    ExpectIdent(TokenStreamState),
    #[error("expect use expr")]
    ExpectUseExpr(TokenStreamState),
    #[error("expect `::`")]
    ExpectScopeResolution(TokenStreamState),
    #[error("expect `}}`")]
    ExpectRightCurlyBrace(TokenStreamState),
    #[error("expect `use` token")]
    ExpectUseToken(TokenStreamState),
    #[error("invalid `*` as use root")]
    InvalidAllAsRoot {
        use_token: UseToken,
        star_token: StarToken,
    },
    #[error("invalid ident as use root")]
    InvalidLeafAsRoot {
        use_token: UseToken,
        ident_token: IdentToken,
    },
    #[error("invalid `self` as use root")]
    InvalidSelfAsRoot {
        use_token: UseToken,
        self_mod_token: SelfModToken,
    },
}

impl OriginalError for OriginalUseExprError {
    type Error = UseExprError;
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
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
            use_expr_arena,
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
        match use_expr {
            UseExpr::All { star_token } => Err(OriginalUseExprError::InvalidAllAsRoot {
                use_token,
                star_token,
            }
            .into()),
            UseExpr::Leaf { ident_token } => Err(OriginalUseExprError::InvalidLeafAsRoot {
                use_token,
                ident_token,
            }
            .into()),
            UseExpr::SelfOne { self_mod_token } => Err(OriginalUseExprError::InvalidSelfAsRoot {
                use_token,
                self_mod_token,
            }
            .into()),
            UseExpr::Parent(_) => Ok(UseExprRoot {
                use_token,
                parent_use_expr_idx: ParentUseExprIdx(self.use_expr_arena.alloc_one(use_expr)),
            }),
            UseExpr::Err(e) => Err(e),
        }
    }

    fn parse_use_expr_after_ident(&mut self, ident_token: IdentToken) -> UseExprResult<UseExpr> {
        let Some(scope_resolution_token) = self.parse::<ScopeResolutionToken>()? else {
            return Ok( UseExpr::Leaf { ident_token })
        };
        Ok(UseExpr::Parent(ParentUseExpr {
            parent_name_token: PathNameToken::Ident(ident_token),
            scope_resolution_token: Ok(scope_resolution_token),
            children: self.parse_children(),
        }))
    }

    fn parse_children(&mut self) -> UseExprResult<UseExprChildren> {
        let Some(lcurl_token) = self.parse::<LeftCurlyBraceToken>()? else {
            let child = self.parse_expected(OriginalUseExprError::ExpectUseExpr).into();
            let child = self.use_expr_arena.alloc_one(child);
            return Ok(UseExprChildren::Single { child })
        };
        let (children, comma_tokens, result) = parsec::parse_separated_list(self);
        if let Err(_e) = result {
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

impl<'a, 'b> ParseFromStream<UseExprParser<'a, 'b>> for UseExpr {
    type Error = UseExprError;
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut UseExprParser<'a, 'b>,
    ) -> UseExprResult<Option<Self>> {
        if let Some(star_token) = ctx.parse::<StarToken>()? {
            return Ok(Some(UseExpr::All { star_token }));
        }
        let Some(path_name_token) = ctx.parse::<PathNameToken>()? else {
            return Ok(None)
        };
        match path_name_token {
            PathNameToken::Ident(ident_token) => {
                Ok(Some(ctx.parse_use_expr_after_ident(ident_token)?))
            }
            PathNameToken::CrateRoot(crate_token) => Ok(Some(UseExpr::Parent(ParentUseExpr {
                parent_name_token: PathNameToken::CrateRoot(crate_token),
                scope_resolution_token: ctx
                    .parse_expected(OriginalUseExprError::ExpectScopeResolution),
                children: ctx.parse_children(),
            }))),
            PathNameToken::SelfMod(self_mod_token) => {
                // differentiate betwee self one and self children
                if ctx.peek() == Some(&Token::Punctuation(Punctuation::COLON_COLON)) {
                    Ok(Some(UseExpr::Parent(ParentUseExpr {
                        parent_name_token: self_mod_token.into(),
                        scope_resolution_token: Ok(ctx
                            .parse()
                            .expect("guaranteed by peek")
                            .expect("guaranteed by peek")),
                        children: ctx.parse_children(),
                    })))
                } else {
                    Ok(Some(UseExpr::SelfOne { self_mod_token }))
                }
            }
            PathNameToken::Super(super_token) => Ok(Some(UseExpr::Parent(ParentUseExpr {
                parent_name_token: PathNameToken::Super(super_token),
                scope_resolution_token: ctx
                    .parse_expected(OriginalUseExprError::ExpectScopeResolution),
                children: ctx.parse_children(),
            }))),
        }
    }
}
