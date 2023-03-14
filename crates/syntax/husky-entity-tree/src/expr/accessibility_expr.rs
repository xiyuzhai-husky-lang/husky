use super::*;
use husky_token::*;
use parsec::*;
use thiserror::Error;

#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum AccessibilityExpr {
    Public {
        pub_token: PubToken,
    },
    PublicUnder {
        pub_token: PubToken,
        lpar: LeftParenthesisToken,
        scope: ModPathExprIdx,
        rpar: RightParenthesisToken,
    },
}

#[derive(Debug, Error)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum AccessibilityExprError {
    #[error("token error")]
    Token(#[from] TokenError),
}

pub type AccessibilityExprResult<T> = Result<T, AccessibilityExprError>;

pub(crate) fn parse_accessibility_expr(
    token_stream: &mut TokenStream,
    mod_path_expr_arena: &mut ModPathExprArena,
) -> AccessibilityExprResult<Option<AccessibilityExpr>> {
    AccessibilityExprParser {
        token_stream,
        mod_path_expr_arena,
    }
    .parse()
}

struct AccessibilityExprParser<'a, 'b> {
    token_stream: &'b mut TokenStream<'a>,
    mod_path_expr_arena: &'b mut ModPathExprArena,
}

impl<'a, 'b> parsec::ParseFrom<AccessibilityExprParser<'a, 'b>> for AccessibilityExpr {
    type Error = AccessibilityExprError;
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut AccessibilityExprParser<'a, 'b>,
    ) -> Result<Option<Self>, AccessibilityExprError> {
        let Some(pub_token) = ctx.parse::<PubToken>()? else {
            return Ok(None)
        };
        let Some(_lpar) = ctx.parse::<LeftParenthesisToken>()? else {
            return Ok(Some(AccessibilityExpr::Public { pub_token }))
        };
        todo!()
    }
}

impl<'a, 'b> std::ops::Deref for AccessibilityExprParser<'a, 'b> {
    type Target = TokenStream<'a>;

    fn deref(&self) -> &Self::Target {
        &self.token_stream
    }
}

impl<'a, 'b> std::ops::DerefMut for AccessibilityExprParser<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token_stream
    }
}

impl<'a, 'b> std::borrow::Borrow<TokenStream<'a>> for AccessibilityExprParser<'a, 'b> {
    fn borrow(&self) -> &TokenStream<'a> {
        &self.token_stream
    }
}

impl<'a, 'b> std::borrow::BorrowMut<TokenStream<'a>> for AccessibilityExprParser<'a, 'b> {
    fn borrow_mut(&mut self) -> &mut TokenStream<'a> {
        &mut self.token_stream
    }
}

impl<'a, 'b> StreamWrapper for AccessibilityExprParser<'a, 'b> {}
