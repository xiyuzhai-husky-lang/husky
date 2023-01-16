use super::*;
use husky_entity_taxonomy::{ModuleItemKind, TypeKind};
use husky_opn_syntax::Bracket;
use husky_token::*;
use parsec::{HasParseError, StreamWrapper};
use std::iter::Peekable;

pub(super) trait AstTokenParseContext<'a>: TokenParseContext<'a>
where
    <Self as HasParseError>::Error: From<TokenError>,
{
    fn ast_context_kind(&self) -> AstContextKind;
    fn module_path(&self) -> ModulePath;

    fn parse_accessibility(&mut self) -> AstResult<Accessibility> {
        let token_stream = &mut self.borrow_mut();
        Ok(match token_stream.peek().unwrap() {
            Token::Attr(decor) => match decor {
                AttributeKeyword::Pub => {
                    token_stream.next();
                    match token_stream
                        .peek()
                        .ok_or(AstError::ExpectParBraOrDecoratorOrIdentifier(None))?
                    {
                        Token::Punctuation(Punctuation::Bra(Bracket::Par)) => todo!(),
                        _ => Accessibility::Public,
                    }
                }
                AttributeKeyword::Protected => todo!(),
                AttributeKeyword::Private => todo!(),
                AttributeKeyword::Async => todo!(),
                AttributeKeyword::Static => Accessibility::Public,
            },
            _ => Accessibility::PublicUnder(self.module_path()),
        })
    }

    fn take_entity_kind_keyword(&mut self) -> AstResult<Keyword> {
        let (idx, token) = self
            .borrow_mut()
            .next_indexed()
            .ok_or(AstError::ExpectEntityKeyword)?;
        Ok(match token {
            Token::Attr(_) => self.take_entity_kind_keyword()?,
            Token::Keyword(kw) => kw,
            _ => return Err(AstError::ExpectEntityKeyword),
        })
    }

    fn parse_is_generic(&mut self) -> bool {
        let Some (token) = &self
            .token_stream_mut()
            .peek() else { return false };
        match token {
            Token::Punctuation(Punctuation::LaOrLt) => true,
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
    fn ast_context_kind(&self) -> AstContextKind {
        self.ast_parent
    }

    fn module_path(&self) -> ModulePath {
        self.module_path
    }
}

pub(crate) struct BasicAuxAstParser<'a> {
    db: &'a dyn AstDb,
    ast_parent: AstContextKind,
    module_path: ModulePath,
    token_iter: TokenStream<'a>,
}

impl<'a> HasParseError for BasicAuxAstParser<'a> {
    type Error = AstError;
}

impl<'a> BasicAuxAstParser<'a> {
    pub(super) fn new(
        db: &'a dyn AstDb,
        ctx: &Context,
        module_path: ModulePath,
        token_iter: TokenStream<'a>,
    ) -> Self {
        Self {
            db,
            ast_parent: ctx.kind(),
            module_path,
            token_iter,
        }
    }

    pub(super) fn finish_with_saved_stream_state(self) -> TokenIdx {
        self.token_iter.state()
    }

    pub(crate) fn db(&self) -> &dyn AstDb {
        self.db
    }
}
