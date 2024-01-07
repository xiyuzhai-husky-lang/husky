use super::*;

use husky_scope_expr::VisibilityExprResult;
use husky_token::*;
use parsec::StreamWrapper;

pub(super) trait AstTokenParseContext<'a>: TokenStreamParser<'a> {
    fn module_path(&self) -> ModulePath;

    fn parse_is_generic(&mut self) -> bool {
        let Some(token) = &self.token_stream_mut().peek() else {
            return false;
        };
        match token {
            TokenData::Punctuation(Punctuation::LA_OR_LT) => true,
            _ => false,
        }
    }
}

impl<'a> std::ops::Deref for BasicAuxAstParser<'a> {
    type Target = TokenStream<'a>;

    fn deref(&self) -> &Self::Target {
        &self.token_iter
    }
}

impl<'a> core::borrow::Borrow<TokenStream<'a>> for BasicAuxAstParser<'a> {
    fn borrow(&self) -> &TokenStream<'a> {
        &self.token_iter
    }
}

impl<'a> core::borrow::BorrowMut<TokenStream<'a>> for BasicAuxAstParser<'a> {
    fn borrow_mut(&mut self) -> &mut TokenStream<'a> {
        &mut self.token_iter
    }
}

impl<'a> std::ops::DerefMut for BasicAuxAstParser<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token_iter
    }
}

impl<'a> StreamWrapper for BasicAuxAstParser<'a> {}

impl<'a> AstTokenParseContext<'a> for BasicAuxAstParser<'a> {
    fn module_path(&self) -> ModulePath {
        self.module_path
    }
}

pub(crate) struct BasicAuxAstParser<'a> {
    db: &'a ::salsa::Db,
    module_path: ModulePath,
    token_iter: TokenStream<'a>,
}

impl<'a> BasicAuxAstParser<'a> {
    pub(super) fn new(
        db: &'a ::salsa::Db,
        module_path: ModulePath,
        token_iter: TokenStream<'a>,
    ) -> Self {
        Self {
            db,
            module_path,
            token_iter,
        }
    }

    pub(super) fn finish_with_saved_stream_state(self) -> TokenStreamState {
        self.token_iter.save_state()
    }

    // todo: where to put this?
    pub(crate) fn parse_visibility_expr(&mut self) -> VisibilityExprResult<VisibilityExpr> {
        VisibilityExpr::parse_from_token_stream(self.db, self.module_path, &mut self.token_iter)
    }
}
