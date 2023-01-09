use husky_opn_syntax::{BinaryOpr, BinaryPureClosedOpr, Bracket};
use husky_token::{TokenParseContext, TokenStream};
use parsec::{HasParseError, StreamWrapper};

use super::*;

impl<'a> AstParser<'a> {
    pub(super) fn parse_uses(&mut self, token_group_idx: TokenGroupIdx, ctx: &Context) -> Ast {
        let token_iter = self
            .token_sheet
            .token_group_token_stream(token_group_idx, None);
        let (ident, mut aux_parser) =
            EntityUseExprParser::new(ctx, token_iter, self.module_path, &mut self.use_expr_arena);
        let accessibility = match aux_parser.parse_accessibility() {
            Ok(accessibility) => accessibility,
            Err(error) => {
                return Ast::Err {
                    token_group_idx,
                    error,
                }
            }
        };
        let use_expr = match aux_parser.parse_step() {
            Some(expr) => expr,
            None => self.use_expr_arena.alloc_one(UseExpr::Err(todo!())),
        };
        Ast::Use {
            token_group_idx,
            ident,
            accessibility,
            use_expr_idx: use_expr,
        }
    }
}

pub struct EntityUseExprParser<'a> {
    token_iter: TokenStream<'a>,
    parent: AstContextKind,
    module_path: ModulePath,
    arena: &'a mut UseExprArena,
}

impl<'a> HasParseError for EntityUseExprParser<'a> {
    type Error = AstError;
}

impl<'a> std::ops::Deref for EntityUseExprParser<'a> {
    type Target = TokenStream<'a>;

    fn deref(&self) -> &Self::Target {
        &self.token_iter
    }
}

impl<'a> std::ops::DerefMut for EntityUseExprParser<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token_iter
    }
}

impl<'a> StreamWrapper for EntityUseExprParser<'a> {}

impl<'a> core::borrow::Borrow<TokenStream<'a>> for EntityUseExprParser<'a> {
    fn borrow(&self) -> &TokenStream<'a> {
        &self.token_iter
    }
}

impl<'a> core::borrow::BorrowMut<TokenStream<'a>> for EntityUseExprParser<'a> {
    fn borrow_mut(&mut self) -> &mut TokenStream<'a> {
        &mut self.token_iter
    }
}

impl<'a> AstTokenParseContext<'a> for EntityUseExprParser<'a> {
    fn ast_context_kind(&self) -> AstContextKind {
        self.parent
    }

    fn module_path(&self) -> ModulePath {
        self.module_path
    }
}

impl<'b> EntityUseExprParser<'b> {
    fn new(
        ctx: &Context,
        mut token_iter: TokenStream<'b>,
        module_path: ModulePath,
        arena: &'b mut UseExprArena,
    ) -> (Identifier, Self) {
        while let Some(token) = token_iter.next() {
            match token {
                Token::Keyword(Keyword::Use) => break,
                _ => continue,
            }
        }
        let ident = match token_iter.peek() {
            Some(token) => match token {
                Token::Attr(_) => todo!(),
                Token::Keyword(_) => todo!(),
                Token::Identifier(ident) => ident,
                Token::Punctuation(_) => todo!(),
                Token::WordOpr(_) => todo!(),
                Token::Literal(_) => todo!(),
                Token::Err(_) => todo!(),
            },
            None => todo!(),
        };
        (
            *ident,
            Self {
                token_iter,
                arena,
                module_path,
                parent: ctx.kind(),
            },
        )
    }

    fn parse_step(&mut self) -> Option<UseExprIdx> {
        let expr = self.parse_step_inner()?;
        Some(self.arena.alloc_one(expr))
    }

    fn parse_step_inner(&mut self) -> Option<UseExpr> {
        let token = self.token_iter.next()?;
        Some(match token {
            Token::Identifier(ident) => {
                let Some(next_token) = self.token_iter.next() else  {
                    return Some(UseExpr::One { ident: *ident })
                };
                match next_token {
                    Token::Punctuation(Punctuation::DoubleColon) => {}
                    Token::Punctuation(Punctuation::Comma)
                    | Token::Punctuation(Punctuation::Ket(Bracket::Curl)) => {
                        self.token_iter.step_back();
                        return Some(UseExpr::One { ident: *ident });
                    }
                    _ => return Some(UseExpr::Err(EntityUseExprError::ExpectScopeResolution)),
                }
                let Some(child) =  self.parse_step() else {
                    return Some(UseExpr::Err(
                        EntityUseExprError::ExpectScopeResolution,
                    ))
                };
                UseExpr::ScopeResolution {
                    parent: *ident,
                    child,
                }
            }
            Token::Punctuation(Punctuation::Bra(Bracket::Curl)) => self.parse_multiple(),
            // ad hoc; todo: change this to SpecialToken::Star
            Token::Punctuation(Punctuation::Binary(BinaryOpr::PureClosed(
                BinaryPureClosedOpr::Mul,
            ))) => UseExpr::All {},
            _ => UseExpr::Err(EntityUseExprError::ExpectSomething),
        })
    }

    fn parse_multiple(&mut self) -> UseExpr {
        let mut exprs: Vec<UseExpr> = vec![];
        loop {
            match self.parse_step_inner() {
                Some(expr) => exprs.push(expr),
                None => todo!(),
            }
            match self.token_iter.next() {
                Some(token) => match token {
                    Token::Punctuation(Punctuation::Comma) => continue,
                    Token::Punctuation(Punctuation::Ket(Bracket::Curl)) => break,
                    _ => todo!(),
                },
                None => exprs.push(todo!()),
            }
        }
        UseExpr::Multiple {
            exprs: self.arena.alloc_batch(exprs),
        }
    }
}
