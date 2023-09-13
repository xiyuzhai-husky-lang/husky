use super::*;
use husky_token::*;
use parsec::*;
use thiserror::Error;

#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum VisibilityExpr {
    Public {
        pub_token: RegionalPubToken,
    },
    PublicUnder {
        pub_token: RegionalPubToken,
        lpar: RegionalLparToken,
        visibility: ModulePathExprIdx,
        rpar: RparToken,
    },
}

#[derive(Debug, Error)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum VisibilityExprError {
    #[error("token error")]
    TokenData(#[from] TokenDataError),
}

pub type VisibilityExprResult<T> = Result<T, VisibilityExprError>;

pub(crate) fn parse_visibility_expr(
    token_stream: &mut TokenStream,
    mod_path_expr_arena: &mut ModulePathExprArena,
) -> VisibilityExprResult<Option<VisibilityExpr>> {
    VisibilityExprParser {
        token_stream,
        mod_path_expr_arena,
    }
    .parse()
}

struct VisibilityExprParser<'a, 'b> {
    token_stream: &'b mut TokenStream<'a>,
    mod_path_expr_arena: &'b mut ModulePathExprArena,
}

impl<'a, 'b> parsec::ParseFrom<VisibilityExprParser<'a, 'b>> for VisibilityExpr {
    type Error = VisibilityExprError;
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut VisibilityExprParser<'a, 'b>,
    ) -> Result<Option<Self>, VisibilityExprError> {
        let Some(pub_token) = ctx.parse::<RegionalPubToken>()? else {
            return Ok(None);
        };
        let Some(_lpar) = ctx.parse::<RegionalLparToken>()? else {
            return Ok(Some(VisibilityExpr::Public { pub_token }));
        };
        todo!()
    }
}

impl<'a, 'b> std::ops::Deref for VisibilityExprParser<'a, 'b> {
    type Target = TokenStream<'a>;

    fn deref(&self) -> &Self::Target {
        &self.token_stream
    }
}

impl<'a, 'b> std::ops::DerefMut for VisibilityExprParser<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token_stream
    }
}

impl<'a, 'b> std::borrow::Borrow<TokenStream<'a>> for VisibilityExprParser<'a, 'b> {
    fn borrow(&self) -> &TokenStream<'a> {
        &self.token_stream
    }
}

impl<'a, 'b> std::borrow::BorrowMut<TokenStream<'a>> for VisibilityExprParser<'a, 'b> {
    fn borrow_mut(&mut self) -> &mut TokenStream<'a> {
        &mut self.token_stream
    }
}

impl<'a, 'b> StreamWrapper for VisibilityExprParser<'a, 'b> {}
